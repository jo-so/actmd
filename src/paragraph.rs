use crate::log;

use std::mem;

use super::{
    Block,
    Inline,
    LINE_WS,
    NL_CR,
    ParserData,
    ParserSettings,
    Pattern,
    Location,
    LocationHelper,
    LocationPosition,
    Tools,
    Transaction,
    html_entity,
    is_ascii_alphabetic,
};

use Inline as I;

mod link;

use link::{link_end, linkdef};

#[derive(Debug)]
enum E {
    Start,
    End,
    Both,
}

#[derive(Debug)]
enum Entity {
    Emph(E, char, (usize, usize), usize),
    Image((usize, usize)),
    Link((usize, usize)),
    NestedLink,
}

struct Paragraph {
    list: Vec<Inline>,
    plain: String,
    plain_begin: LocationPosition,
    open_brackets: Vec<Entity>,
    open_braces: u16,
}

impl Paragraph {
    fn new(plain_begin: LocationPosition) -> Self {
        Self {
            list: Default::default(),
            plain: Default::default(),
            plain_begin,
            open_brackets: Default::default(),
            open_braces: Default::default(),
        }
    }

    fn push(&mut self, el: Inline, plain_end: LocationPosition) {
        self.push_plain(plain_end);
        log!(t, "paragraph", "adding {:?}", el);
        self.list.push(el);
    }

    fn push_no_plain(&mut self, el: Inline) {
        assert!(self.plain.is_empty());
        log!(t, "paragraph", "adding {:?}", el);
        self.list.push(el);
    }

    fn push_plain(&mut self, plain_begin: LocationPosition) {
        if !self.plain.is_empty() {
            let begin = mem::replace(&mut self.plain_begin, plain_begin);
            // let end = begin + self.plain.len();
            let end = plain_begin;

            let el = I::Plain(mem::take(&mut self.plain), Location { begin, end });

            log!(t, "paragraph", "adding {:?}", el);
            self.list.push(el);
        }
    }

    fn push_char(&mut self, ch: char) {
        self.plain.push(ch);
    }

    fn into_list(self) -> Vec<Inline> {
        let mut list = self.list;
        if !self.plain.is_empty() {
            let end = self.plain_begin + self.plain.len();

            list.push(I::Plain(
                self.plain,
                Location {
                    begin: self.plain_begin,
                    end,
                }
            ));
        }

        list
    }
}

pub fn paragraph(data: &mut impl ParserData, stop_on_emb_end: bool) -> Block {
    let par_loc_begin = data.loc();
    log!(d, data, "paragraph", "begin");
    data.skip_all(LINE_WS);

    let mut par = Paragraph::new(data.loc());

    while let Some(peek_ch) = data.peek() {
        match peek_ch {
            '\n' | '\r' => {
                par.push_plain(data.loc());
                data.skip_newline();
                data.skip_all(LINE_WS);

                if data.peek().is_none() {
                    break;
                }

                par.push_no_plain(I::SoftBreak);
                par.plain_begin = data.loc();
            }

            '`' => {
                let loc = data.loc();
                if let Some(code) = code(data) {
                    par.push(I::Code(code), loc);
                    par.plain_begin = data.loc();
                } else {
                    data.copy_all(&mut par.plain, '`');
                }
            }

            '<' => {
                if let Some((url, txt, loc, inner_loc)) = autourl(data) {
                    par.push(
                        I::Link(
                            vec![I::Plain(txt, inner_loc)],
                            url,
                            String::new(),
                            // TODO: loc
                        ),
                        data.loc()
                    );
                } else if !data.has_setting(ParserSettings::Html) {
                    par.push_char('<');
                    data.advance();
                } else {
                    let plain_end = data.loc();

                    if let Some(html) = html(data) {
                        par.push_plain(plain_end);
                        par.list.extend(html);
                        par.plain_begin = data.loc()
                    } else {
                        par.push_char('<');
                        data.advance();
                    }
                }
            }

            // image begin
            '!' => {
                data.advance();
                let pos_in_plain = par.plain.len();
                par.push_char('!');

                if data.skip('[') {
                    par.push_char('[');

                    log!(d, data, "paragraph", "image begin");
                    par.open_brackets.push(
                        Entity::Image((par.list.len(), pos_in_plain))
                    );
                }
            }

            // link begin
            '[' => {
                if par.list.is_empty() && par.plain.is_empty() {
                    if let Some(ld) = linkdef(data) {
                        return ld;
                    }
                }

                data.advance();
                log!(d, data, "paragraph", "link begin");
                par.open_brackets.push(Entity::Link( (par.list.len(), par.plain.len()) ));
                par.push_char('[');
            }

            ']' => link_end(&mut par, data),

            '*' | '_' => emph(&mut par, data),

            '@' if data.has_setting(ParserSettings::Embedded) => {
                data.advance();

                if let Some((em, loc)) = embedded(data, true) {
                    let el = match em {
                        Embedded::Expr(x) => I::EmbeddedExpr(x, loc),

                        Embedded::Block(x, br) => {
                            par.open_braces += br;
                            I::EmbeddedBlock(x, loc)
                        }
                    };

                    par.push(el, data.loc());
                } else {
                    par.push_char('@');
                }
            }

            '}' if /* TODO */ false && data.has_setting(ParserSettings::Embedded) => {
                if par.open_braces > 0 {
                    let loc_begin = data.loc();
                    data.advance();
                    par.open_braces -= 1;

                    if Some(&I::SoftBreak) == par.list.last() {
                        par.list.pop();
                    }

                    let loc = data.loc_end(loc_begin);
                    data.skip_all(' ');
                    par.push(I::EmbeddedBlock("}".to_string(), loc), data.loc());
                } else if stop_on_emb_end {
                    break;
                } else {
                    par.push_char('}');
                    data.advance();
                }
            }

            // https://spec.commonmark.org/0.29/#entity-and-numeric-character-references
            '&' => html_entity(data, &mut par.plain),

            '\\' => {
                let loc_before_bs = data.loc();
                data.advance();

                match data.peek() {
                    // https://spec.commonmark.org/0.29/#hard-line-breaks
                    Some('\r') | Some('\n') => {
                        data.skip_newline();

                        if data.peek().is_none() {
                            par.push_char('\\');
                            par.push_plain(loc_before_bs + 1);
                            break;
                        }
                        par.push_plain(loc_before_bs);
                        par.push_no_plain(I::HardBreak);

                        data.skip_all(LINE_WS);
                        par.plain_begin = data.loc();
                    }

                    // https://spec.commonmark.org/0.29/#backslash-escapes
                    Some(ch) if ch.is_ascii_punctuation() => {
                        par.push_char(ch);
                        data.advance();
                    }

                    _ => par.push_char('\\'),
                }
            }

            c if LINE_WS.matches(c) => {
                let loc = data.loc();
                let ws_start = par.plain.len();
                data.copy_all(&mut par.plain, LINE_WS);

                if data.looking_at(NL_CR) || data.peek().is_none() {
                    par.plain.truncate(ws_start);
                    par.push_plain(loc);
                }
            }

            c => {
                par.push_char(c);
                data.advance();
            }
        }
    }

    let mut emph_found = false;
    for e in &mut par.open_brackets {
        use Entity::*;

        match e {
            Link(..) | Image(..) => *e = NestedLink,
            Emph(..) => emph_found = true,
            NestedLink => (),
        }
    }

    if emph_found {
        link::emph_cleanup(&mut par, data.loc());
    }

    log!(d, data, "paragraph", "end");
    Block::Paragraph(par.into_list(), data.loc_end(par_loc_begin))
}

fn autourl(data: &mut impl ParserData) -> Option<(String, String, Location, Location)> {
    if !data.looking_at('<') {
        return None;
    }

    let mut data = Transaction::new(data);
    let loc_begin = data.loc();
    let mut url = String::new();
    let mut is_mailto = true;
    let mut is_url = true;
    let mut colon_pos = None;
    let mut at_pos = None;

    loop {
        match data.next()? {
            '>' => {
                data.advance();
                break;
            }

            '<' => return None,
            c if !c.is_ascii_graphic() => return None,

            ':' if colon_pos.is_none() => {
                if at_pos.is_some() {
                    // @ before : is not allowed in schema => no URL
                    // : after @ is not allowed in an mailto => no mailto
                    return None;
                }

                let pos = url.len();
                colon_pos = Some(pos);
                if pos < 2 || pos > 32 {
                    // schema to short or to long
                    is_url = false;
                }

                url.push(':');
            }

            '@' => {
                if at_pos.is_none() {
                    let pos = url.len();
                    at_pos = Some(pos);
                    if pos == 0 || colon_pos.is_some() {
                        // a : is not allowed in a mailto
                        is_mailto = false;
                    }
                } else {
                    // only one @ is allowed in a mailto
                    is_mailto = false;
                }

                url.push('@');
            }

            ch => {
                if is_url {
                    let invalid_url_begin = url.is_empty() && !ch.is_ascii_alphanumeric();
                    let invalid_scheme_char = colon_pos.is_none() &&
                    // restriction of characters in scheme
                        !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '.' || ch == '+');

                    if invalid_url_begin || invalid_scheme_char {
                        is_url = false;
                    }
                }

                if is_mailto {
                    if at_pos.is_none() {
                        match ch {
                            // restriction of characters before @
                            '(' | ')' | '[' | ']' | '"' | ',' | ';' | '\\' => is_mailto = false,
                            _ => (),
                        }
                    } else if !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '.')
                    // restriction of characters after @
                    {
                        is_mailto = false;
                    }
                }

                url.push(ch);
            }
        }

        if !is_url && !is_mailto {
            return None;
        }
    }

    if let Some(p) = at_pos {
        if url.len() - p == 1 {
            is_mailto = false;
        }
    } else {
        is_mailto = false;
    }

    if colon_pos.is_none() {
        is_url = false;
    }

    let loc = data.loc_end(loc_begin);
    let inner_loc = Location {
        begin: loc.begin + 1,
        end: loc.end - 1,
    };

    if is_url {
        data.commit();
        Some((url.clone(), url, loc, inner_loc))
    } else if is_mailto {
        data.commit();
        Some((String::from("mailto:") + &url, url, loc, inner_loc))
    } else {
        None
    }
}

fn clip_segment(
    inlines: &mut Vec<Inline>, start: (usize, usize), delim_len: usize,
    end: Option<usize>
) -> Vec<Inline> {
    trait AsPlain {
        fn as_plain(&mut self) -> (&mut String, &mut Location);
    }

    impl AsPlain for I {
        fn as_plain(&mut self) -> (&mut String, &mut Location) {
            if let I::Plain(txt, loc) = self {
                (txt, loc)
            } else {
                unreachable!("Found {:?} instead of Plain", self);
            }
        }
    }

    if start.1 == 0 {
        let mut drop_first = false;

        let (txt, loc) = inlines[start.0].as_plain();
        if txt.len() == delim_len {
            drop_first = true;
        } else if delim_len == 1 {
            txt.remove(0);
            loc.begin = loc.begin + 1;
        } else {
            txt.replace_range(0..delim_len, "");
            loc.begin = loc.begin + delim_len;
        }

        let mut drain = if let Some(end) = end {
            inlines.drain(start.0 .. end)
        } else {
            inlines.drain(start.0..)
        };

        if drop_first {
            assert!(drain.next().is_some());
        }

        drain.collect()
    } else if start.1 == inlines[start.0].as_plain().0.len() - delim_len {
        let (txt, loc) = inlines[start.0].as_plain();
        txt.truncate(start.1);
        loc.end = loc.end - delim_len;

        if let Some(end) = end {
            inlines.drain(start.0 + 1 .. end)
        } else {
            inlines.drain(start.0 + 1..)
        }.collect()
    } else {
        let (src, loc) = inlines[start.0].as_plain();
        let plain = src[start.1 + delim_len..].to_string();
        src.truncate(start.1);

        let plain_loc = Location {
            begin: loc.begin + start.1 + delim_len,
            end: loc.end,
        };
        loc.end = loc.begin + start.1;

        let mut content = vec![ I::Plain(plain, plain_loc) ];
        if let Some(end) = end {
            if start.0 + 1 < end {
                content.extend(inlines.drain(start.0 + 1 .. end));
            }
        } else {
            content.extend(inlines.drain(start.0 + 1..));
        }

        content
    }
}

fn code(data: &mut impl ParserData) -> Option<String> {
    let mut data = Transaction::new(data);
    let tag_len = data.skip_all('`');
    if tag_len == 0 {
        return None;
    }

    let mut text = String::new();

    loop {
        match data.peek()? {
            '`' => {
                let len = data.skip_all('`');

                if len == tag_len {
                    break;
                }

                for _ in 0..len {
                    text.push('`');
                }
            }

            '\r' | '\n' => {
                data.skip_newline();
                text.push(' ');
            }

            c => {
                text.push(c);
                data.advance();
            }
        }
    }

    if text.len() > 2 && text.starts_with(' ') && text.ends_with(' ') {
        text.pop();
        text.remove(0);
    }

    data.commit();
    Some(text)
}

pub enum Embedded {
    Block(String, u16),
    Expr(String),
}

pub(crate) fn embedded(data: &mut impl ParserData, foo: bool) -> Option<(Embedded, Location)> {
    let pos = data.pos();
    let loc_begin = data.loc();

    match data.peek()? {
        '{' => {
            data.advance();
            let mut buf = String::new();
            if data.copy_until_match(&mut buf, '{', '}') {
                buf.pop();
                Some((Embedded::Block(buf, 0), data.loc_end(loc_begin)))
            } else {
                data.reset(pos).unwrap();
                None
            }
        }

        '(' => {
            data.advance();
            let mut buf = String::new();
            if data.copy_until_match(&mut buf, '(', ')') {
                buf.pop();
                Some((Embedded::Expr(buf), data.loc_end(loc_begin)))
            } else {
                data.reset(pos).unwrap();
                None
            }
        }

        '\n' | '\r' => {
            data.skip_newline();
            data.skip_all(LINE_WS);

            Some((Embedded::Block(String::from("//\n"), 0), data.loc_end(loc_begin)))
        }

        '/' => {
            match data.next()? {
                '/' => {
                    let mut buf = String::from("/");

                    data.copy_until(&mut buf, NL_CR);
                    if buf.ends_with('\r') && data.skip('\n') {
                        buf.push('\n');
                    }

                    data.skip_all(LINE_WS);

                    Some((Embedded::Block(buf, 0), data.loc_end(loc_begin)))
                }

                '*' => {
                    data.advance();

                    let mut buf = String::from("/*");
                    data.copy_until_seq(&mut buf, "*/");
                    Some((Embedded::Block(buf, 0), data.loc_end(loc_begin)))
                }

                _ => {
                    data.reset(pos).unwrap();
                    None
                }
            }
        }

        c if foo && (c.is_ascii_alphanumeric() || "_&".contains(c)) => {
            let mut buf = String::new();
            data.copy_all(
                &mut buf, |c: char| c.is_ascii_alphanumeric() || "_&:.".contains(c)
            );

            match buf.as_str() {
                "if" | "for" | "while" | "loop" => {
                    let buf_len = buf.len();
                    if data.copy_until(&mut buf, '{') {
                        let mut braces = 1;
                        loop {
                            let pos = data.pos();
                            let buf_len = buf.len();

                            data.copy_all(&mut buf, &[' ', '\t', '\n', '\r'][..]);

                            let word_start = buf.len();
                            if data.copy_all(&mut buf, is_ascii_alphabetic) > 1 {
                                match &buf[word_start..] {
                                    "if" | "for" | "while" | "loop" => {
                                        if data.copy_until(&mut buf, '{') {
                                            braces += 1;
                                            continue;
                                        }
                                    }

                                    _ => (),
                                }
                            }

                            data.reset(pos).unwrap();
                            buf.truncate(buf_len);
                            break;
                        }

                        Some((Embedded::Block(buf, braces), data.loc_end(loc_begin)))
                    } else {
                        data.reset(pos).unwrap();
                        buf.truncate(buf_len);
                        None
                    }
                }

                _ => {
                    if data.looking_at('(') {
                        let pos = data.pos();
                        data.advance();
                        buf.push('(');

                        if data.copy_until_match(&mut buf, '(', ')') {
                            Some((Embedded::Expr(buf), data.loc_end(loc_begin)))
                        } else {
                            data.reset(pos).unwrap();
                            buf.pop();
                            Some((Embedded::Expr(buf), data.loc_end(loc_begin)))
                        }
                    } else {
                        Some((Embedded::Expr(buf), data.loc_end(loc_begin)))
                    }
                }
            }
        }

        _ => {
            data.reset(pos).unwrap();
            None
        }
    }
}

fn emph(par: &mut Paragraph, data: &mut impl ParserData) {
    #![allow(clippy::nonminimal_bool)]
    /*
      This function contains some expressions taken from the Commonmark Spec,
      hence they should be kept as is. Unfortunately, it's (currently) not
      possible to disable this warning only for those lines.
     */

    let delim_ch = match data.peek() {
        Some(c @ '*') | Some(c @ '_') => c,
        x => unreachable!("Invalid start of emph: {:?}", x),
    };

    log!(d, data, "emph", "begin");

    let delim_len = data.skip_all(delim_ch);
    let prev_char = par.plain.chars().last()
    // the delimiter comes after an content element (image, link, ...)
    // https://spec.commonmark.org/0.29/#example-403
        .or_else(|| par.list.last().map(|x| match x {
            I::HardBreak | I::SoftBreak => ' ',
            _ => '.',
        }));
    let (is_left_side, is_right_side) = emph_delimiter_evaluation(prev_char, data.peek());

    let is_opening = if delim_ch == '*' {
        // rule 1. + 5.
        is_left_side && !is_right_side
    } else {
        // rule 2. + 6.
        is_left_side && (!is_right_side || (
            is_right_side && prev_char.map_or(false, |c| c.is_ascii_punctuation())
        ))
    };


    let mut is_closing = if delim_ch == '*' {
        // rule 3. + 7.
        is_right_side && !is_left_side
    } else {
        // rule 4. + 8.
        is_right_side && (!is_left_side || (
            is_left_side && data.peek().map_or(false, |c| c.is_ascii_punctuation())
        ))
    };

    log!(t, "emph", "opening = {}, closing = {}", is_opening, is_closing);

    let is_both = is_opening == is_closing;
    if delim_ch == '*' && !is_opening && !is_closing &&
        is_left_side && is_right_side
    {
        // intraword emphasis
        // https://spec.commonmark.org/0.29/#example-369
        // https://spec.commonmark.org/0.29/#example-416
        is_closing = true;
    }

    if is_closing {
        let remaining_chars = emph_end(par, delim_ch, delim_len, is_both);

        if remaining_chars > 0 {
            if is_both {
                par.open_brackets.push(Entity::Emph(
                    E::Both, delim_ch,
                    (par.list.len(), par.plain.len()),
                    remaining_chars
                ));
            }

            for _ in 0..remaining_chars {
                par.push_char(delim_ch);
            }
        }

        par.plain_begin = data.loc() - par.plain.len();
    } else {
        if is_opening {
            par.open_brackets.push(Entity::Emph(
                E::Start, delim_ch,
                (par.list.len(), par.plain.len()),
                delim_len
            ));
        }

        for _ in 0..delim_len {
            par.push_char(delim_ch);
        }
    }

    log!(d, data, "emph", "end");
}

fn emph_delimiter_evaluation(prev: Option<char>, next: Option<char>) -> (bool, bool) {
    #![allow(clippy::nonminimal_bool)]
    /*
      This function contains some expressions taken from the Commonmark Spec,
      hence they should be kept as is. Unfortunately, it's (currently) not
      possible to disable this warning only for those lines.
     */

    let (ws_fol, punc_fol) = match next {
        None => (true, false),
        Some(c) => (c.is_whitespace(), c.is_ascii_punctuation()),
    };
    let (ws_prec, punc_prec) = match prev {
        None => (true, false),
        Some(c) => (c.is_whitespace(), c.is_ascii_punctuation()),
    };

    let left = !ws_fol && (!punc_fol || (punc_fol && (ws_prec || punc_prec)));
    let right = !ws_prec && (!punc_prec || (punc_prec && (ws_fol || punc_fol)));

    (left, right)
}

fn emph_end(
    par: &mut Paragraph, delim_ch: char, closing_len: usize, closing_is_both: bool
) -> usize {
    let mut matching_start_idx = None;
    let mut open_link_found = false;

    for (idx, e) in par.open_brackets.iter().enumerate().rev() {
        match e {
            Entity::Emph(emph, c, _, l) => {
                let opening_is_both = match emph {
                    E::Both => true,
                    E::Start => false,
                    E::End => continue,
                };

                if *c == delim_ch && (
                    (!opening_is_both && !closing_is_both) ||
                    // rule 9. + 10. from
                    // https://spec.commonmark.org/0.29/#emphasis-and-strong-emphasis
                    // https://spec.commonmark.org/0.29/#example-410
                        (l + closing_len) % 3 != 0 ||
                        (l % 3 == 0 && closing_len % 3 == 0)
                ) {
                    matching_start_idx = Some(idx);
                    break;
                }
            }

            Entity::NestedLink => (),

            _ => open_link_found = true,
        }
    }

    if matching_start_idx.is_none() {
        return closing_len;
    }

    let (opening_pos, opening_len);

    if let Some(idx) = matching_start_idx {
        use Entity::Emph;

        if open_link_found {
            // push a marker in the list, maybe the link becomes invalid
            par.open_brackets.push(Emph(
                E::End, delim_ch, (par.list.len(), par.plain.len()), closing_len
            ));

            for _ in 0..closing_len {
                par.plain.push(delim_ch);
            }

            return 0;
        }

        match &par.open_brackets[idx] {
            Emph(E::Start, _, s, l) | Emph(E::Both, _, s, l) => {
                opening_pos = *s;
                opening_len = *l;
            }

            x => unreachable!("{:?}", x),
        }

        let mut i = 0;
        par.open_brackets.retain(|x| {
            let idx = idx + 1;
            i += 1;
            match x {
                // there should be no Ends left, because !open_link_found
                Emph(E::End, ..) => panic!("{:?} found", x),

                // remove other open EmphStart: `*a _b*`
                Emph(..) => i < idx,

                _ => true
            }
        });
    } else {
        return closing_len;
    };

    // TODO: besseren Namen
    fn foo(par: &mut Paragraph, pos: (usize, usize), len: usize) {
        par.push_plain(Default::default());
        let ct = clip_segment(&mut par.list, pos, len, None);

        if len == 1 {
            par.push_no_plain(Inline::Emph(ct));
        } else {
            let mut new = Inline::Strong(ct);
            let mut l = len - 2;

            while l >= 2 {
                new = Inline::Strong(vec![new]);
                l -= 2;
            }

            if l == 1 {
                new = Inline::Emph(vec![new]);
            }

            par.push_no_plain(new);
        }
    }

    if opening_len <= closing_len {
        foo(par, opening_pos, opening_len);

        if opening_len < closing_len {
            return emph_end(par, delim_ch, closing_len - opening_len, closing_is_both);
        }
    } else {
        par.open_brackets.push(
            Entity::Emph(E::Start, delim_ch, opening_pos, opening_len - closing_len)
        );

        let pos = (opening_pos.0, opening_pos.1 + opening_len - closing_len);

        foo(par, pos, closing_len);
    }

    0
}

fn html(data: &mut impl ParserData) -> Option<Vec<Inline>> {
    #[inline]
    fn is_ascii_uppercase(ch: char) -> bool { char::is_ascii_uppercase(&ch) }

    if !data.looking_at('<') {
        return None;
    }

    log!(d, data, "inline html", "begin");

    let loc_begin = data.loc();
    let mut data = Transaction::new(data);
    let mut html = String::from("<");
    let mut list = Vec::new();

    macro_rules! push {
        () => (
            if !html.is_empty() {
                log!(t, data, "inline html", "adding HTML");
                list.push(Inline::Html(
                    mem::take(&mut html), data.loc_end(loc_begin)
                ));
            }
        );

        ($el:expr) => (
            push!();
            list.push($el);
        );
    }

    macro_rules! leave {
        () => ({
            log!(w, data, "inline html", "end without match");
            return None;
        });
    }

    macro_rules! unwrap {
        ($val:expr) => (
            match $val {
                None => leave!(),
                Some(x) => x,
            }
        );
    }

    match unwrap!(data.next()) {
        '?' => {
            html.push('?');
            data.advance();
            log!(d, data, "inline html", "processing instruction");
            if !data.copy_until_seq(&mut html, "?>") {
                leave!();
            }
        }

        '!' => {
            html.push('!');

            match unwrap!(data.next()) {
                '-' => {
                    // <!-- comment
                    log!(d, data, "inline html", "comment");
                    html.push('-');
                    data.advance();

                    if data.skip('-') {
                        html.push('-');
                    } else {
                        leave!();
                    }

                    if data.looking_at('>') {
                        leave!();
                    }

                    if data.copy_until_seq(&mut html, "--") && data.skip('>') {
                        html.push('>');
                    } else {
                        leave!();
                    }
                    log!(d, data, "inline html", "comment end");
                }

                '[' => {
                    // <![CDATA[
                    log!(d, data, "inline html", "cdata");
                    data.copy_all(&mut html, |c| is_ascii_uppercase(c) || c == '[');
                    if html != "<![CDATA[" {
                        leave!();
                    }

                    if !data.copy_until_seq(&mut html, "]]>") {
                        leave!();
                    }
                }

                _ => {
                    // declaration
                    log!(d, data, "inline html", "declaration");
                    if !(data.copy_all(&mut html, is_ascii_uppercase) > 0
                         && LINE_WS.matches(unwrap!(data.peek()))
                         && data.copy_until(&mut html, '>')
                    ) {
                        leave!();
                    }
                }
            }
        }

        c => {
            let is_opening_tag = c != '/';
            if !is_opening_tag {
                html.push('/');
                data.advance();
            }

            if !data.looking_at(is_ascii_alphabetic) {
                leave!();
            }

            data.copy_all(&mut html, |c: char| c.is_ascii_alphanumeric() || c == '-');
            log!(d, data, "inline html", "general tag \"{}\"", html);
            let space_after_name = data.copy_all(&mut html, " \t\n\r") > 0;

            if is_opening_tag {
                if space_after_name {
                    loop {
                        match unwrap!(data.peek()) {
                            '/' | '>' => break,

                            c if c.is_ascii_alphabetic() || c == '_' || c == ':' => (),

                            _ => leave!(),
                        }

                        log!(t, data, "inline html", "attribute begin");
                        data.copy_all(
                            &mut html,
                            |c: char| c.is_ascii_alphanumeric() || "_.:-".contains(c)
                        );
                        let space_after_name = data.copy_all(&mut html, LINE_WS) > 0;

                        log!(t, data, "inline html", "attribute end");
                        if data.skip('=') {
                            html.push('=');
                        } else if space_after_name {
                            continue;
                        } else {
                            break;
                        }

                        data.copy_all(&mut html, LINE_WS);
                        log!(t, data, "inline html", "value begin");

                        match unwrap!(data.peek()) {
                            ch @ '"' | ch @ '\'' => {
                                html.push(ch);
                                data.advance();

                                if data.has_setting(ParserSettings::Embedded) {
                                    let mut val_open_braces = 0;

                                    loop {
                                        data.copy_until(&mut html, &[ch, '@', '}'][..]);

                                        if html.ends_with(ch) {
                                            break;
                                        } else if html.ends_with('@') {
                                            if let Some((code, loc)) = embedded(&mut data, true) {
                                                html.pop();

                                                push!(
                                                    match code {
                                                        Embedded::Expr(x) => I::EmbeddedExpr(x, loc),

                                                        Embedded::Block(x, br) => {
                                                            val_open_braces += br;
                                                            I::EmbeddedBlock(x, loc)
                                                        }
                                                    }
                                                );
                                            }
                                        } else if html.ends_with('}') {
                                            if val_open_braces > 0 {
                                                val_open_braces -= 1;

                                                push!(Inline::EmbeddedBlock(
                                                    "}".to_string(),
                                                    data.loc_end(data.loc() - 1)
                                                ));

                                                data.skip_all(' ');
                                            }
                                        } else {
                                            leave!();
                                        }
                                    }
                                } else if !data.copy_until(&mut html, ch) {
                                    leave!();
                                }
                            }

                            _ => {
                                if data.copy_all(
                                    &mut html, |c| !"\"'=<>` \t\n\r".contains(c)
                                ) == 0 {
                                    leave!();
                                }
                            }
                        }

                        log!(t, data, "inline html", "value end");
                        if data.copy_all(&mut html, " \t\n\r") == 0 {
                            break;
                        }
                    }
                }

                if data.skip('/') {
                    html.push('/');
                }
            }

            if data.skip('>') {
                html.push('>');
            } else {
                leave!();
            }
        }
    }

    log!(d, data, "inline html", "end");
    if !html.is_empty() {
        list.push(Inline::Html(html, data.loc_end(loc_begin)));
    }
    data.commit();

    Some(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emph_delimiter_left_side() {
        // from https://spec.commonmark.org/0.29/#emphasis-and-strong-emphasis

        assert_eq!((true, false), emph_delimiter_evaluation(None, Some('a')));
        assert_eq!((true, false), emph_delimiter_evaluation(Some(' '), Some('_')));
        assert_eq!((true, false), emph_delimiter_evaluation(None, Some('"')));
        assert_eq!((true, false), emph_delimiter_evaluation(Some(' '), Some('"')));
    }

    #[test]
    fn emph_delimiter_right_side() {
        // from https://spec.commonmark.org/0.29/#emphasis-and-strong-emphasis

        assert_eq!((false, true), emph_delimiter_evaluation(Some('c'), None));
        assert_eq!((false, true), emph_delimiter_evaluation(Some('c'), Some(' ')));
        assert_eq!((false, true), emph_delimiter_evaluation(Some('"'), None));
        assert_eq!((false, true), emph_delimiter_evaluation(Some('"'), Some(' ')));
    }

    #[test]
    fn emph_delimiter_both() {
        // from https://spec.commonmark.org/0.29/#emphasis-and-strong-emphasis

        assert_eq!((true, true), emph_delimiter_evaluation(Some('c'), Some('d')));
        assert_eq!((true, true), emph_delimiter_evaluation(Some('"'), Some('"')));
    }

    #[test]
    fn emph_delimiter_neither() {
        // from https://spec.commonmark.org/0.29/#emphasis-and-strong-emphasis

        assert_eq!((false, false), emph_delimiter_evaluation(Some(' '), Some(' ')));
    }
}
