use crate::log;

use std::{
    convert::TryFrom,
    mem,
};

use super::{
    Block,
    Inline,
    LINE_WS,
    Location,
    LocationHelper,
    LocationPosition,
    NL_CR,
    ParserData,
    ParserSettings,
    Pattern,
    Position,
    Tools,
    Transaction,
    body,
    html_entity,
    is_ascii_alphabetic,
    paragraph::{self, paragraph},
};

pub fn block(
    data: &mut impl ParserData,
    list: &mut Vec<Block>,
    open_embedded_codes: &mut u16
) {
    struct InnerData<'a> {
        inner: &'a mut dyn ParserData,
        ignore_char: Option<char>,
        stop_pos: Option<Position>,
        next_block: Option<Block>,
    }

    impl<'a> InnerData<'a> {
        fn new(inner: &'a mut dyn ParserData, ignore_char: Option<char>) -> Self {
            Self {
                inner,
                ignore_char,
                stop_pos: None,
                next_block: None,
            }
        }

        fn do_hint(self, list: &mut Vec<Block>) {
            if let Some(blk) = self.next_block {
                list.push(blk);
            }
        }
    }

    impl ParserData for InnerData<'_> {
        fn settings(&self) -> ParserSettings {
            self.inner.settings()
        }

        fn pos(&self) -> Position {
            self.stop_pos.unwrap_or(self.inner.pos())
        }

        fn peek(&self) -> Option<char> {
            if self.stop_pos.is_some() {
                None
            } else {
                self.inner.peek()
            }
        }

        fn advance(&mut self) {
            if self.stop_pos.is_some() {
                return;
            }

            let data = &mut self.inner;
            let prev = data.peek().unwrap();

            data.advance();

            match data.peek() {
                None => {
                    log!(t, data, "block", "stopped");
                    self.stop_pos = Some(data.pos());
                }

                // two successive newlines
                Some('\n') if prev == '\n' => {
                    log!(t, data, "block", "stopped");
                    self.stop_pos = Some(data.pos());
                }

                Some('\r') if prev == '\r' || prev == '\n' => {
                    log!(t, data, "block", "stopped");
                    self.stop_pos = Some(data.pos());
                }

                // inside a \r\n
                Some('\n') if prev == '\r' => (),

                // after newline
                Some(mut ch) if prev == '\r' || prev == '\n' => {
                    let after_newline = data.pos();

                    macro_rules! stop {
                        () => ({
                            log!(t, data, "block", "stopped {}", after_newline);
                            self.stop_pos = Some(after_newline);
                        });
                    }

                    let indent = data.skip_all(LINE_WS);

                    if indent > 0 {
                        if data.looking_at(NL_CR) {
                            stop!();
                            return;
                        } else if let Some(c) = data.peek() {
                            ch = c;
                        } else {
                            stop!();
                            return;
                        }
                    }

                    match ch {
                        c if self.ignore_char.map_or(false, |ig| c == ig) => (),

                        '>' => stop!(),

                        '#' => {
                            if let Some((lvl, ct, loc)) = heading(data) {
                                self.next_block = Some(Block::Heading(lvl, ct, loc));
                                stop!();
                            } else if indent > 0 {
                                data.reset(after_newline).unwrap();
                            }
                        }

                        '`' | '~' => {
                            if let Some((info, ct, loc)) = code_fenced(data, indent) {
                                self.next_block = Some(Block::Code(info, ct, loc));
                                stop!();
                            } else if indent > 0 {
                                data.reset(after_newline).unwrap();
                            }
                        }

                        '*' | '-' | '_' if thematic_break(data) => {
                            self.next_block = Some(Block::Break);
                            stop!();
                        }

                        '*' | '-' | '+' => {
                            let pos = data.pos();
                            data.advance();

                            if data.looking_at(LINE_WS) {
                                stop!();
                                data.reset(after_newline).unwrap()
                            } else {
                                data.reset(pos).unwrap()
                            }
                        }

                        '1' => {
                            let pos = data.pos();
                            data.advance();

                            if data.skip('.') && data.looking_at(LINE_WS) {
                                stop!();
                                data.reset(after_newline).unwrap();
                            } else {
                                data.reset(pos).unwrap()
                            }
                        }

                        _ => {
                            if indent > 0 {
                                data.reset(after_newline).unwrap();
                            }
                        }
                    }
                }

                _ => ()
            }
        }

        fn reset(&mut self, pos: Position) -> Result<(), ()> {
            self.inner.reset(pos)?;
            if self.stop_pos.map_or(false, |p| pos < p) {
                self.stop_pos = None;
            }
            Ok(())
        }
    }

    log!(d, data, "block", "begin");

    let (peek_char, indent) = loop {
        while data.skip_newline() { }

        // TODO: muss abhängig von TAB gezählt werden.
        let indent = data.skip_all(LINE_WS);

        match data.peek() {
            None => return,
            Some('\n') | Some('\r') => continue,
            Some(c) => break (c, indent),
        }
    };

    log!(t, data, "block", "first char");

    match peek_char {
        '#' => {
            if let Some((lvl, ct, loc)) = heading(data) {
                list.push(Block::Heading(lvl, ct, loc));
                log!(d, data, "block", "end");
                return;
            }
        }

        '>' => {
            let mut data = InnerData::new(data, Some('>'));

            if let Some((ct, loc)) = quote(&mut data) {
                list.push(Block::Quote(ct, loc));
                log!(d, data, "block", "end");
                data.do_hint(list);
                return;
            }
        }

        '`' | '~' => {
            if let Some((info, ct, loc)) = code_fenced(data, indent) {
                list.push(Block::Code(info, ct, loc));
                log!(d, data, "block", "end");
                return;
            }
        }

        '*' | '-' | '_' if thematic_break(data) => {
            list.push(Block::Break);
            log!(d, data, "block", "end");
            return;
        }

        '*' | '-' | '+' => {
            if let Some(ul) = unordered_list(data) {
                list.push(Block::UnorderedList(ul));
                log!(d, data, "block", "end");
                return;
            }
        }

        '0' ..= '9' => {
            if let Some((start_no, ol)) = ordered_list(data) {
                list.push(Block::OrderedList(start_no, ol));
                log!(d, data, "block", "end");
                return;
            }
        },

        '@' if data.has_setting(ParserSettings::Embedded) => {
            if let Some(blk) = embedded(data, open_embedded_codes) {
                list.push(blk);
                log!(d, data, "block", "end");
                return;
            }
        }

        '}' if *open_embedded_codes > 0 => {
            let mut data = Transaction::new(data);
            let mut buf = String::from('}');
            let loc_begin = data.loc();

            data.advance();
            if data.skip_newline() || data.peek().is_none() {
                *open_embedded_codes -= 1;
                list.push(Block::EmbeddedBlock(buf, data.loc_end(loc_begin)));

                log!(d, data, "block", "end");
                data.commit();
                return;
            }

            data.copy_all(&mut buf, LINE_WS);

            let start = buf.len();
            data.copy_all(&mut buf, &['e', 'l', 's'][..]);
            if &buf[start..] == "else" {
                data.copy_all(&mut buf, LINE_WS);

                if data.skip('{') {
                    buf.push('{');

                    if data.skip_newline() {
                        list.push(Block::EmbeddedBlock(buf, data.loc_end(loc_begin)));
                        log!(d, data, "block", "end");
                        data.commit();
                        return;
                    }
                }
            }
        }

        '<' if data.has_setting(ParserSettings::Html) => {
            if html(data, indent, list, open_embedded_codes) {
                log!(d, data, "block", "end");
                return;
            }
        }

        _ => (),
    }

    let mut data = InnerData::new(data, None);

    list.push( paragraph(&mut data, *open_embedded_codes > 0) );
    log!(d, data, "block", "end");
    data.do_hint(list);
}

fn code_fenced(data: &mut impl ParserData, indent: usize) -> Option<(String, String, Location)> {
    let fence_char = match data.peek() {
        Some(c @ '`') | Some(c @ '~') => c,
        _ => return None,
    };

    let loc_begin = data.loc();
    let mut data = Transaction::new(data);

    let fence_len = data.skip_all(fence_char);
    if fence_len < 3 {
        return None;
    }

    data.skip_all(LINE_WS);
    log!(d, data, "fenced code block", "begin");

    let mut info = String::new();
    loop {
        match data.peek() {
            None => {
                let loc = data.loc_end(loc_begin);
                log!(d, data, "fenced code block", "end");
                data.commit();
                return Some((info, String::new(), loc));
            }

            Some('\r') | Some('\n') => {
                data.skip_newline();
                break;
            }

            Some('`') if fence_char == '`' => {
                log!(d, data, "fenced code block", "end");
                return None;
            }

            Some('&') => html_entity(&mut data, &mut info),

            Some('\\') => {
                data.advance();

                info.push(
                    data.peek().and_then(|c| if c.is_ascii_punctuation() {
                        data.advance();
                        Some(c)
                    } else {
                        None
                    }).unwrap_or('\\')
                );
            }

            Some(c) => {
                data.advance();
                info.push(c);
            }
        }
    }

    while info.ends_with(&LINE_WS[..]) {
        info.pop();
    }

    let mut ct = String::new();

    'out: loop {
        let spaces_before = data.skip_all(' ').saturating_sub(indent);
        if spaces_before < 4 {
            let f_len = data.skip_all(fence_char);
            if f_len >= fence_len {
                let spaces_after = data.skip_all(' ');
                if data.skip_newline() || data.peek().is_none() {
                    break;
                }

                for _ in 0..spaces_before { ct.push(' '); }
                for _ in 0..f_len { ct.push(fence_char); }
                for _ in 0..spaces_after { ct.push(' '); }
            } else {
                for _ in 0..spaces_before { ct.push(' '); }
                for _ in 0..f_len { ct.push(fence_char); }
            }
        } else {
            for _ in 0..spaces_before { ct.push(' '); }
        }

        loop {
            match data.peek() {
                None => {
                    if !ct.is_empty() && !ct.ends_with(&['\n', '\r'][..]) {
                        ct.push('\n');
                    }
                    break 'out;
                }
                Some('\r') => {
                    ct.push('\r');
                    data.advance();

                    match data.peek() {
                        None => break 'out,
                        Some('\n') => {
                            ct.push('\n');
                            data.advance();
                        },
                        _ => (),
                    }
                    break;
                }
                Some('\n') => {
                    ct.push('\n');
                    data.advance();
                    break;
                }
                Some(c) => {
                    ct.push(c);
                    data.advance();
                },
            }
        }
    }

    let loc = data.loc_end(loc_begin);
    log!(d, data, "fenced code block", "end");
    data.commit();
    Some((info, ct, loc))
}

fn embedded(data: &mut impl ParserData, open_embedded_codes: &mut u16) -> Option<Block> {
    if !data.looking_at('@') {
        return None;
    }

    log!(d, data, "embedded block", "begin");
    let mut loc_begin = data.loc();

    let ch = match data.next() {
        None => {
            log!(d, data, "embedded block", "end with plain");

            return Some(Block::Paragraph(
                vec![Inline::Plain("@".into(), data.loc_end(loc_begin))],
                data.loc_end(loc_begin),
            ));
        }

        Some('\r') | Some('\n') => {
            data.skip_newline();
            data.skip_all(LINE_WS);

            return Some(Block::EmbeddedBlock(
                "// \\n".into(),
                data.loc_end(loc_begin),
            ));
        }

        Some('@') => {
            // @@ forces embedded code in paragraph
            log!(d, data, "embedded block", "end without match");
            return None;
        }

        Some(ch) => ch,
    };

    loc_begin = data.loc();
    data.advance();

    let mut buf = String::new();
    let loc_end;

    if ch == '{' && data.skip_newline() {
        loc_begin = data.loc();

        loop {
            if data.peek().is_none() {
                loc_end = data.loc();
                break;
            }

            data.copy_until(&mut buf, NL_CR);
            if buf.ends_with('\r') && data.skip('\n') {
                buf.push('\n');
            }

            let loc = data.loc();
            if data.skip('}') {
                if data.skip_newline() {
                    loc_end = loc;
                    break;
                }

                buf.push('}');
            }
        }
    } else {
        buf.push(ch);
        data.copy_until(&mut buf, NL_CR);
        let line_end = buf.len() - 1;

        if buf.ends_with('\r') && data.skip('\n') {
            buf.push('\n');
        }

        if buf.starts_with("/*") && !buf[..line_end].ends_with("*/") {
            loop {
                data.copy_until(&mut buf, '/');

                if data.peek().is_none() {
                    loc_end = data.loc();
                    break;
                }

                let loc = data.loc();
                if (buf.ends_with("\n*/") || buf.ends_with("\r*/"))
                    && data.skip_newline()
                {
                    loc_end = loc;
                    break;
                }
            }
        } else if buf[..line_end].ends_with('{') {
            *open_embedded_codes += 1;
            loc_end = data.loc();
        } else if buf[..line_end].ends_with('(') {
            loop {
                data.copy_until(&mut buf, ')');

                if data.peek().is_none() {
                    loc_end = data.loc();
                    break;
                }

                let loc = data.loc();
                if (buf.ends_with("\n)") || buf.ends_with("\r)"))
                    && data.skip_newline()
                {
                    loc_end = loc;
                    break;
                }
            }
        } else {
            loc_end = data.loc();
        }
    }

    log!(d, data, "embedded block", "end");
    Some(Block::EmbeddedBlock(buf, Location { begin: loc_begin, end: loc_end }))
}

fn heading(data: &mut impl ParserData) -> Option<(u8, Vec<Inline>, Location)> {
    struct InnerData<'a, T: ParserData> {
        inner: &'a mut T,
        stopped: bool,
    }

    impl<T: ParserData> ParserData for InnerData<'_, T> {
        fn settings(&self) -> ParserSettings {
            self.inner.settings()
        }

        fn pos(&self) -> Position {
            self.inner.pos()
        }

        fn peek(&self) -> Option<char> {
            if self.stopped {
                None
            } else {
                self.inner.peek()
            }
        }

        fn advance(&mut self) {
            if self.stopped {
                return;
            }

            self.inner.advance();

            if self.inner.peek().is_none() || self.inner.skip_newline() {
                log!(t, self.inner, "heading", "stopped");
                self.stopped = true;
            }
        }

        fn reset(&mut self, pos: Position) -> Result<(), ()> {
            self.inner.reset(pos)?;
            self.stopped = false;
            Ok(())
        }
    }

    let loc_begin = data.loc();
    log!(d, data, "heading", "begin");

    let level;
    {
        let mut data = Transaction::new(data);

        level = u8::try_from(data.skip_all('#')).ok()?;
        if level > 6 {
            log!(d, data, "heading", "end without match");
            return None;
        }

        if data.peek().is_none() || data.looking_at(NL_CR) {
            let loc = data.loc_end(loc_begin);
            log!(d, data, "heading", "end");
            data.commit();

            return Some((level, Vec::new(), loc));
        }

        if !data.looking_at(LINE_WS) {
            log!(d, data, "heading", "end without match");
            return None
        }

        data.commit();
    }

    let pos = data.pos();
    let mut data = InnerData {
        inner: data,
        stopped: false,
    };

    let par = paragraph(&mut data, false);

    assert!(data.stopped, "paragraph() stopped before EOL");

    let (ct, loc) = match par {
        Block::Paragraph(ct, mut loc) => {
            loc.begin = loc_begin;

            log!(d, data, "heading", "end");
            (ct, loc)
        }

        Block::LinkDef(..) => {
            let data = data.inner;
            data.reset(pos).unwrap();
            let inner_loc_begin = data.loc();

            let mut buf = String::new();
            data.copy_all(&mut buf, |c| !NL_CR.matches(c));
            let loc = data.loc_end(loc_begin);
            let inner_loc = data.loc_end(inner_loc_begin);
            data.skip_newline();

            log!(d, data, "heading", "end");
            (vec![Inline::Plain(buf, inner_loc)], loc)
        }

        x => unreachable!("{:?}", x),
    };

    Some((level, ct, loc))
}

// from https://spec.commonmark.org/0.29/#html-blocks
const HTML_TAG_NAMES : &[&str] = &[
    "address", "article", "aside", "base", "basefont", "blockquote", "body",
    "caption", "center", "col", "colgroup", "dd", "details", "dialog", "dir",
    "div", "dl", "dt", "fieldset", "figcaption", "figure", "footer", "form",
    "frame", "frameset", "h1", "h2", "h3", "h4", "h5", "h6", "head", "header",
    "hr", "html", "iframe", "legend", "li", "link", "main", "menu", "menuitem",
    "meta", "nav", "noframes", "ol", "optgroup", "option", "p", "param",
    "section", "source", "summary", "table", "tbody", "td", "templateX", "tfoot",
    "th", "thead", "title", "tr", "track", "ul"
];

fn html(
    data: &mut impl ParserData, indent: usize, list: &mut Vec<Block>, op_br: &mut u16
) -> bool {
    #[inline]
    fn is_ascii_uppercase(ch: char) -> bool { char::is_ascii_uppercase(&ch) }

    fn finish_with_blank_line(
        mut data: Transaction<impl ParserData>, mut buf: String,
        list: &mut Vec<Block>, op_br: &mut u16, loc_begin: LocationPosition
    ) {
        let stop_chars = if data.has_setting(ParserSettings::Embedded) {
            &['\r', '\n', '@'][..]
        } else {
            &NL_CR[..]
        };

        loop {
            if data.peek().is_none() {
                if !buf.is_empty() && !buf.ends_with('\n') && !buf.ends_with('\r') {
                    buf.push('\n');
                }

                break;
            }

            data.copy_until(&mut buf, stop_chars);

            if buf.ends_with('@') {
                let loc_end = data.loc();

                if data.skip('@') {
                    // @@ should become @ for escaping
                } else if let Some((x, loc)) = paragraph::embedded(&mut data, false) {
                    buf.pop();

                    list.push(Block::Html(
                        mem::take(&mut buf),
                        Location { begin: loc_begin, end: loc_end }
                    ));

                    match x {
                        paragraph::Embedded::Expr(x) => {
                            list.push(Block::EmbeddedExpr(x, loc));
                        }

                        paragraph::Embedded::Block(x, br) => {
                            *op_br += br;
                            list.push(Block::EmbeddedBlock(x, loc));
                        }
                    }
                }
            } else {
                if buf.ends_with('\r') && data.skip('\n') {
                    buf.push('\n');
                }

                if data.looking_at(NL_CR) {
                    break;
                }

                if data.has_setting(ParserSettings::Embedded) && (
                    data.looking_at('@') || (*op_br > 0 && data.looking_at('}'))
                ) {
                    break;
                }
            }
        }

        list.push(Block::Html(buf, data.loc_end(loc_begin)));

        log!(d, data, "html block", "end");
        data.commit();
    }

    fn finish_with_eol(
        mut data: Transaction<impl ParserData>, mut buf: String,
        list: &mut Vec<Block>, loc_begin: LocationPosition
    ) {
        data.copy_until(&mut buf, NL_CR);
        if buf.ends_with('\r') {
            if data.skip('\n') {
                buf.push('\n');
            }
        } else if !buf.ends_with('\n') {
            buf.push('\n');
        }

        list.push(Block::Html(buf, data.loc_end(loc_begin)));

        log!(d, data, "html block", "end");
        data.commit();
    }

    if !data.looking_at('<') {
        return false;
    }

    log!(d, data, "html block", "begin");
    let loc_begin = data.loc() - indent;

    let mut data = Transaction::new(data);
    data.advance();

    let mut buf = String::new();
    for _ in 0..indent { buf.push(' '); }
    buf.push('<');

    let is_closing_tag = data.skip('/');
    let is_opening_tag = !is_closing_tag;
    if is_closing_tag {
        buf.push('/');
    }

    let start_len = buf.len();
    if data.copy_all(&mut buf, is_ascii_alphabetic) == 0 {
        if is_closing_tag {
            // a closing tag must have an attribute name
            log!(w, data, "html block", "end without match");
            return false;
        }

        if data.skip('!') {
            buf.push('!');

            let start = buf.len();
            data.copy_all(
                &mut buf, |c| is_ascii_uppercase(c) || c == '!' || c == '-' || c == '['
            );

            let foo = &buf[start..];
            if foo.starts_with("--") {
                // case 2: <!--
                log!(t, data, "html block", "comment");

                if data.skip('>') {
                    buf.push('>');
                } else {
                    data.copy_until_seq(&mut buf, "-->");
                }
            } else if foo.starts_with(is_ascii_uppercase) {
                // case 4: <!DOCTYPE
                log!(t, data, "html block", "DOCTYPE");
                data.copy_until(&mut buf, '>');
            } else if foo.starts_with("[CDATA[") {
                // case 5: <![CDATA[
                log!(t, data, "html block", "CDATA");
                data.copy_until_seq(&mut buf, "]]>");
            } else {
                log!(w, data, "html block", "end without match");
                return false;
            }
        } else if data.looking_at('?') {
            // case 3: <?
            log!(t, data, "html block", "processing instruction");
            data.copy_until_seq(&mut buf, "?>");
        } else {
            log!(w, data, "html block", "end without match");
            return false;
        }

        finish_with_eol(data, buf, list, loc_begin);
        true
    } else {
        let name = buf[start_len..].to_ascii_lowercase();
        let name = &name.as_str();

        if HTML_TAG_NAMES.binary_search(name).is_ok() {
            // case 6
            log!(t, data, "html block", "tag \"{}\"", name);
            finish_with_blank_line(data, buf, list, op_br, loc_begin);
            return true;
        } else if is_opening_tag && ["script", "pre", "style"].contains(name) {
            // case 1
            log!(t, data, "html block", "tag \"{}\"", name);

            loop {
                if !data.copy_until_seq(&mut buf, "</") {
                    debug_assert_eq!(None, data.peek());

                    if !buf.ends_with(NL_CR) {
                        buf.push('\n');
                    }

                    list.push(Block::Html(buf, data.loc_end(loc_begin)));
                    log!(d, data, "html block", "end");
                    data.commit();
                    return true;
                }

                let start = buf.len();
                data.copy_all(&mut buf, is_ascii_alphabetic);

                let name = buf[start..].to_ascii_lowercase();
                let name = &name.as_str();
                if ["script", "pre", "style"].contains(name) && data.skip('>') {
                    buf.push('>');
                    break;
                }
            }

            finish_with_eol(data, buf, list, loc_begin);
            return true;
        }

        // case 7 (general tags)
        data.copy_all(&mut buf, |c: char| c.is_ascii_alphanumeric() || c == '-');
        let pos = data.pos();
        log!(t, data, "html block", "tag \"{}\"", buf);

        let space_after_name = data.skip_all(LINE_WS) > 0;
        if is_opening_tag {
            if space_after_name {
                loop {
                    match data.peek() {
                        Some('/') | Some('>') => break,

                        Some(c) if c.is_ascii_alphabetic() || c == '_' || c == ':' => (),

                        _ => {
                            log!(w, data, "html block", "end without match");
                            return false;
                        }
                    }

                    // skip attribute name
                    log!(t, data, "html block", "tag attribute");
                    data.skip_all(|c| char::is_ascii_alphanumeric(&c) || "_.:-".contains(c));
                    let space_after_name = data.skip_all(' ') > 0;

                    log!(t, data, "html block", "tag attribute end");
                    if !data.skip('=') {
                        // there's no attribute value
                        if space_after_name {
                            // go for the next attribute
                            continue;
                        } else {
                            // we should have reached the tag end ... check the end
                            break;
                        }
                    }

                    data.skip_all(' ');
                    log!(t, data, "html block", "tag value");
                    match data.peek() {
                        // missing HTML attribute's value
                        None => {
                            log!(w, data, "html block", "end without match");
                            return false;
                        }

                        Some(ch @ '"') | Some(ch @ '\'') => {
                            data.advance();
                            data.skip_all(|c| c != ch && c != '\r' && c != '\n');

                            if !data.skip(ch) {
                                // missing right delimiter of HTML attribute's value
                                log!(w, data, "html block", "end without match");
                                return false;
                            }
                        }

                        _ => {
                            if data.skip_all(|c| !" \t\"'=<>`".contains(c)) == 0 {
                                // bad HTML attribute's value
                                log!(w, data, "html block", "end without match");
                                return false;
                            }
                        }
                    }

                    log!(t, data, "html block", "tag value end");
                    if data.skip_all(' ') == 0 {
                        break;
                    }
                }
            }

            data.skip('/');
        }

        if !data.skip('>') {
            log!(w, data, "html block", "end without match");
            return false;
        }

        data.skip_all(' ');

        if data.looking_at(NL_CR) || data.peek().is_none() {
            data.reset(pos).unwrap();
            finish_with_blank_line(data, buf, list, op_br, loc_begin);

            true
        } else {
            log!(w, data, "html block", "end without match");
            false
        }
    }
}

fn list_items(
    data: &mut impl ParserData,
    extra_indent: u8,
    skip_marker: impl Fn(&mut dyn ParserData) -> bool
) -> Option<Vec<Vec<Block>>> {
    if data.peek().is_none() {
        log!(d, data, "list items", "end");
        return Some(vec![ Vec::with_capacity(0) ]);
    }

    if data.skip_newline() {
        if data.peek().is_none() || data.skip_newline() {
            log!(d, data, "list items", "end");
            return Some(vec![ Vec::with_capacity(0) ]);
        }

        if !data.skip(LINE_WS) {
            log!(w, data, "list items", "end without match");
            return None;
        }

        log!(e, data, "list items", "with empty line");
    }

    if !data.skip(LINE_WS) {
        log!(w, data, "list items", "end without match");
        return None;
    }

    struct InnerData<'a> {
        inner: &'a mut dyn ParserData,
        stopped: bool,
        extra_indent: u8,
    }

    impl ParserData for InnerData<'_> {
        fn settings(&self) -> ParserSettings {
            self.inner.settings()
        }

        fn pos(&self) -> Position {
            self.inner.pos()
        }

        fn peek(&self) -> Option<char> {
            if self.stopped {
                None
            } else {
                self.inner.peek()
            }
        }

        fn advance(&mut self) {
            if self.stopped {
                return;
            }

            let data = &mut self.inner;
            let prev = data.peek().unwrap();

            data.advance();

            match data.peek() {
                None => {
                    log!(t, data, "unordered list", "stopped");
                    self.stopped = true;
                }

                Some(ch) if !NL_CR.matches(ch) && NL_CR.matches(prev) => {
                    let mut ident = self.extra_indent;
                    while ident > 0 && data.skip(' ') {
                        ident -= 1;
                    }

                    // after newline
                    if !data.skip(LINE_WS) {
                        log!(t, data, "unordered list", "stopped");
                        self.stopped = true;
                        self.extra_indent -= ident;
                        return;
                    }

                    if ch == ' ' && !data.skip(' ') {
                        log!(t, data, "unordered list", "stopped");
                        self.stopped = true;
                        self.extra_indent += 1;
                    }
                }

                _ => (),
            }
        }

        fn reset(&mut self, pos: Position) -> Result<(), ()> {
            self.inner.reset(pos)?;
            self.stopped = false;
            Ok(())
        }
    }

    let mut list = Vec::new();
    let mut data = InnerData {
        inner: data,
        stopped: false,
        extra_indent,
    };

    loop {
        log!(t, data, "list item", "begin");

        list.push( body(&mut data) );

        {
            let data = &mut data.inner;
            let pos = data.pos();

            log!(t, data, "list item", "end");

            if !skip_marker(*data) {
                break;
            }

            if data.peek().is_none() {
                list.push(Vec::with_capacity(0));
                break;
            }

            if !data.looking_at(LINE_WS) {
                if data.looking_at(NL_CR) {
                    log!(e, data, "list item", "start with empty line");
                } else {
                    data.reset(pos).unwrap();
                    break;
                }
            }
        }

        data.stopped = false;

        if data.extra_indent > 1 {
            log!(e, data, "list item", "with extra indent of {}", data.extra_indent);
        }
    }

    Some(list)
}

fn ordered_list(data: &mut impl ParserData) -> Option<(String, Vec<Vec<Block>>)> {
    log!(d, data, "ordered list", "begin");

    fn is_ascii_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    let pos = data.pos();
    let mut first_no = String::new();

    if !(
        (1..10).contains(&data.copy_all(&mut first_no, is_ascii_digit))
            && data.skip(['.', ')'])
    ) {
        log!(w, data, "ordered list", "end without match");
        data.reset(pos).unwrap();
        return None;
    }

    if let Some(list) = list_items(
        data, 1, |d| d.skip_all("0123456789") > 0 && d.skip(['.', ')'])
    ) {
        log!(d, data, "ordered list", "end");
        Some((first_no, list))
    } else {
        data.reset(pos).unwrap();
        None
    }
}

fn quote(data: &mut impl ParserData) -> Option<(Vec<Block>, Location)> {
    if !data.skip('>') {
        return None;
    }

    log!(d, data, "quote", "begin");
    let loc_begin = data.loc() - 1;

    data.skip(LINE_WS);

    struct InnerData<'a> {
        inner: &'a mut dyn ParserData,
        stopped: bool,
    }

    impl ParserData for InnerData<'_> {
        fn settings(&self) -> ParserSettings {
            self.inner.settings()
        }

        fn pos(&self) -> Position {
            self.inner.pos()
        }

        fn peek(&self) -> Option<char> {
            if self.stopped {
                None
            } else {
                self.inner.peek()
            }
        }

        fn advance(&mut self) {
            if self.stopped {
                return;
            }

            let prev = self.inner.peek().unwrap();

            self.inner.advance();

            match self.inner.peek() {
                None => {
                    log!(t, self.inner, "quote", "stopped");
                    self.stopped = true;
                }

                // inside a \r\n
                Some('\n') if prev == '\r' => (),

                // after newline
                Some('>') if NL_CR.matches(prev) => {
                    self.inner.advance();
                    self.inner.skip(LINE_WS);
                }

                Some(' ') | Some('\t') if NL_CR.matches(prev) => {
                    let start_pos = self.inner.pos();
                    self.inner.skip_all(LINE_WS);

                    match self.inner.peek() {
                        None => {
                            log!(t, self.inner, "quote", "stopped");
                            self.stopped = true;
                        }

                        Some('>') => {
                            self.inner.advance();
                            self.inner.skip(LINE_WS);
                        }

                        _ => self.inner.reset(start_pos).unwrap(),
                    }
                }

                _ => ()
            }
        }

        fn reset(&mut self, pos: Position) -> Result<(), ()> {
            self.inner.reset(pos)?;
            self.stopped = false;
            Ok(())
        }
    }

    let mut data = InnerData {
        inner: data,
        stopped: false,
    };

    let content = body(&mut data);

    log!(d, data, "quote", "end");
    Some((content, data.loc_end(loc_begin)))
}

fn thematic_break(data: &mut impl ParserData) -> bool {
    let marker = match data.peek() {
        Some(c @ '*') | Some(c @ '-') | Some(c @ '_') => c,
        _ => return false,
    };

    log!(d, data, "thematic break", "begin");
    let mut data = Transaction::new(data);
    data.advance();

    let mut cnt = 1;
    loop {
        data.skip_all(LINE_WS);

        let c = data.skip_all(marker);
        if c > 0 {
            cnt += c;
        } else if data.skip_newline() || data.peek().is_none() {
            if cnt >= 3 {
                log!(d, data, "thematic break", "end");

                data.commit();
                return true;
            } else {
                log!(w, data, "thematic break", "end without match");

                return false;
            }
        } else {
            log!(w, data, "thematic break", "end without match");
            return false
        }
    }
}

fn unordered_list(data: &mut impl ParserData) -> Option<Vec<Vec<Block>>> {
    log!(d, data, "unordered list", "begin");

    let marker = match data.peek() {
        Some(c @ '*') | Some(c @ '-') | Some(c @ '+') => c,
        _ => return None,
    };

    let pos = data.pos();
    data.advance();

    if let Some(list) = list_items(data, 0, |d| d.skip(marker)) {
        log!(d, data, "unordered list", "end");
        Some(list)
    } else {
        data.reset(pos).unwrap();
        None
    }
}
