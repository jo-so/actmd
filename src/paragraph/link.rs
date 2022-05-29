use crate::log;

use super::{
    Block,
    E,
    Entity,
    Inline as I,
    LINE_WS,
    LocationHelper,
    NL_CR,
    Paragraph,
    ParserData,
    Location,
    LocationPosition,
    Tools,
    Transaction,
    clip_segment,
    html_entity,
};

fn is_segment_empty(par: &Paragraph, par_begin: usize, plain_begin: usize) -> bool {
    if par.list.len() == par_begin {
        return par.plain.len() == plain_begin;
    }

    if let I::Plain(txt, _) = &par.list[par_begin] {
        if txt.len() > plain_begin {
            return false;
        }
    } else {
        unreachable!("Expected Inline::Plain, but found {:?}", par.list[par_begin]);
    }

    par.list[par_begin + 1..].iter()
        .all(|x| matches!(x, I::HardBreak | I::SoftBreak))
}

fn link_arg(data: &mut impl ParserData) -> Option<(String, String)> {
    let mut data = Transaction::new(data);

    if !data.skip('(') {
        return None;
    }

    log!(t, data, "link arg", "begin");

    data.skip_all(LINE_WS);
    if data.skip_newline() {
        data.skip_all(LINE_WS);

        if data.looking_at(NL_CR) {
            return None;
        }
    }

    let url = link_arg_url(&mut data)?;

    let mut space_seen = data.skip_all(LINE_WS) > 0;

    if data.skip_newline() {
        space_seen = true;
        data.skip_all(LINE_WS);
    }

    let title;
    if space_seen {
        title = link_arg_title(&mut data)?;

        if data.skip_newline() {
            data.skip_all(LINE_WS);
        }
    } else {
        title = String::with_capacity(0);
    }

    if data.skip(')') {
        log!(t, data, "link arg", "end");
        data.commit();
        Some((url, title))
    } else {
        log!(w, data, "link end", "end without match");
        None
    }
}

fn link_arg_url(data: &mut impl ParserData) -> Option<String> {
    let mut url = String::new();

    log!(t, data, "link url", "begin");

    if data.looking_at('<') {
        loop {
            match data.next()? {
                '>' => {
                    data.advance();
                    log!(t, data, "link url", "end");
                    return Some(url);
                }

                '\\' => {
                    let c = data.next()?;
                    if !c.is_ascii_punctuation() {
                        url.push('\\')
                    }
                    url.push(c);
                }

                '&' => html_entity(data, &mut url),

                '<' | '\r' | '\n' => {
                    log!(w, data, "link url", "end without match");
                    return None;
                }

                c => url.push(c),
            }
        }
    }

    let mut open_parentheses = 0;
    loop {
        match data.peek() {
            Some(')') => {
                if open_parentheses == 0 {
                    log!(t, data, "link url", "end");
                    return Some(url);
                } else {
                    open_parentheses -= 1;
                    url.push(')');
                    data.advance();
                }
            }

            Some('\\') => {
                let c = data.next()?;
                if !c.is_ascii_punctuation() {
                    url.push('\\')
                }
                url.push(c);
                data.advance();
            }

            Some('&') => html_entity(data, &mut url),

            None | Some(' ') | Some('\t') | Some('\n') | Some('\r') => {
                return if open_parentheses == 0 {
                    log!(t, data, "link url", "end");
                    Some(url)
                } else {
                    log!(w, data, "link url", "end without match");
                    None
                };
            }

            Some(c) => {
                if c == '(' {
                    open_parentheses += 1;
                } else if c < ' ' {
                    log!(w, data, "link url", "end without match");
                    return None;
                }

                data.advance();
                url.push(c);
            }
        }
    }
}

fn link_arg_title(data: &mut impl ParserData) -> Option<String> {
    let (left_del, right_del) = match data.peek() {
        Some(del @ '\'') | Some(del @ '"') => (del, del),

        Some('(') => ('(', ')'),

        _ => return Some(String::with_capacity(0)),
    };

    log!(d, data, "link title", "begin");
    data.advance();

    let mut title = String::new();
    loop {
        match data.peek()? {
            '\\' => {
                let c = data.next()?;
                if !c.is_ascii_punctuation() {
                    title.push('\\')
                }
                title.push(c);
                data.advance();
            }

            '&' => html_entity(data, &mut title),

            c @ '\r' | c @ '\n' => {
                title.push(c);
                data.advance();

                if c == '\r' && data.skip('\n') {
                    title.push('\n');
                }

                data.copy_all(&mut title, LINE_WS);
                if data.looking_at(NL_CR) {
                    log!(w, data, "link title", "end without match");
                    return None;
                }
            }

            c => {
                // check right_del before left_del, because they might be the same
                if c == right_del {
                    data.advance();
                    data.skip_all(LINE_WS);

                    log!(d, data, "link title", "end");
                    return Some(title);
                } else if c == left_del {
                    log!(w, data, "link title", "end without match");
                    return None;
                } else {
                    title.push(c);
                    data.advance();
                }
            }
        }
    }
}

pub(super) fn link_end(par: &mut Paragraph, data: &mut impl ParserData) {
    let plain_end_loc = data.loc();
    data.expect_char(']');

    log!(d, data, "link end", "begin");

    let mut match_ = None;
    let mut nested_link_found = false;
    let mut emph_found = false;
    for (idx, e) in par.open_brackets.iter_mut().enumerate().rev() {
        match e {
            Entity::NestedLink => nested_link_found = true,
            Entity::Emph(..) => emph_found = true,

            Entity::Image(_) | Entity::Link(_) => {
                match_ = Some(idx);
                break;
            }
        }
    }

    log!(
        d, "link end",
        "nested_link_found = {}, emph_found = {}, match = {:?}",
        nested_link_found, emph_found, match_
    );

    fn cleanup_par(par: &mut Paragraph, idx: usize, keep_element: bool) {
        let idx = idx + 1;
        let mut i = 0;
        par.open_brackets.retain(|x| {
            i += 1;
            match x {
                // remove all open emph after/inside the matched image/label/link
                Entity::Emph(..) => i < idx,

                _ => keep_element || i != idx,
            }
        });
    }

    if data.looking_at('(') {
        use Entity as E;

        match match_.map(|i| (i, &par.open_brackets[i])) {
            Some((idx, E::Image(pos))) => {
                if let Some((url, title)) = link_arg(data) {
                    let pos = *pos; // drop reference to par by copying pos

                    par.push_plain(plain_end_loc);
                    let content = clip_segment(&mut par.list, pos, 2, None);

                    par.push_no_plain(I::Image(content, url, title));

                    cleanup_par(par, idx, false);
                    par.plain_begin = data.loc();

                    log!(d, data, "link end", "end");
                    return;
                }
            }

            Some((idx, E::Link(pos))) if !nested_link_found => {
                if let Some((url, title)) = link_arg(data) {
                    let pos = *pos; // drop reference to par by copying pos

                    par.push_plain(plain_end_loc);
                    let content = clip_segment(&mut par.list, pos, 1, None);

                    par.push_no_plain(I::Link(content, url, title));
                    par.open_brackets[idx] = E::NestedLink;

                    cleanup_par(par, idx, true);
                    par.plain_begin = data.loc();

                    log!(d, data, "link end", "end");
                    return;
                }
            }

            _ => (),
        }
    }

    match match_.map(|i| (i, &par.open_brackets[i])) {
        Some((idx, Entity::Image(pos))) => {
            let before_label = data.pos();
            let label = link_label(data);

            if label.as_ref().map_or(false, |x| !x.trim().is_empty()) ||
                (!emph_found && !is_segment_empty(par, pos.0, pos.1 + 2))
            {
                let pos = *pos; // drop reference to par by copying pos
                par.push_plain(plain_end_loc);
                let content = clip_segment(&mut par.list, pos, 2, None);

                par.push_no_plain(I::ImageRef(
                    content,
                    label.unwrap_or_else(|| String::with_capacity(0)),
                ));

                cleanup_par(par, idx, false);
                par.plain_begin = data.loc();

                log!(d, data, "link end", "end with imageref");
                return;
            } else {
                data.reset(before_label).unwrap();
            }
        }

        Some((idx, Entity::Link(pos))) if !nested_link_found => {
            let before_label = data.pos();
            let label = link_label(data);

            if label.as_ref().map_or(false, |x| !x.trim().is_empty()) ||
                (!emph_found && !is_segment_empty(par, pos.0, pos.1 + 1))
            {
                let pos = *pos; // drop reference to par by copying pos
                par.push_plain(plain_end_loc);
                let content = clip_segment(&mut par.list, pos, 1, None);

                par.push_no_plain(I::LinkRef(
                    content,
                    label.unwrap_or_else(|| String::with_capacity(0)),
                ));

                cleanup_par(par, idx, false);
                par.plain_begin = data.loc();

                log!(d, data, "link end", "end with linkref");
                return;
            } else {
                data.reset(before_label).unwrap();
            }
        }

        _ => (),
    }

    if let Some(idx) = match_ {
        par.open_brackets.remove(idx);

        emph_cleanup(par, data.loc());

        log!(d, data, "link end", "end with plain");
    } else {
        log!(w, data, "link end", "end without match");
    }

    par.push_char(']');
}

pub(super) fn emph_cleanup(par: &mut Paragraph, plain_begin: LocationPosition) {
    let mut closed_emph = Vec::new();
    let mut emph_pairs = Vec::new();
    for (idx, e) in par.open_brackets.iter().enumerate().rev() {
        use Entity::*;

        match e {
            Image(..) | Link(..) => break,

            Emph(kind, ..) => match kind {
                E::End => closed_emph.push(idx),

                _ => {
                    if let Some(x) = closed_emph.pop() {
                        emph_pairs.push((idx, x));
                    } else {
                        break;
                    }
                }
            }

            NestedLink => (),
        }
    }

    if emph_pairs.is_empty() {
        log!(t, "emph cleanup", "end without match");
        return;
    }

    log!(i, "emph cleanup", "emph_pairs = {:?}", emph_pairs);
    par.push_plain(plain_begin);

    let mut lowest_start = None;
    let mut end_offset = 0;
    for (start_idx, end_idx) in emph_pairs {
        use Entity::*;

        let (start, end, delim_len) = match &par.open_brackets[start_idx] {
            Emph(E::Start, start_delim, start_pos, start_len) |
            Emph(E::Both, start_delim, start_pos, start_len) => {
                match &par.open_brackets[end_idx] {
                    Emph(E::End, del, pos, len) => {
                        assert_eq!(del, start_delim);
                        assert_eq!(len, start_len);

                        (start_pos, pos, *len)
                    }

                    x => unreachable!("end_idx should be an Emph(End): {:?}", x)
                }
            }

            x => unreachable!(
                "start_idx should be a starting Emph(): {:?}", x
            )
        };

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

        let end = (end.0 + end_offset, end.1);
        let (mut ct, ins_pos);
        if start.0 == end.0 {
            if start.1 == 0 {
                ins_pos = end.0;

                let (text, text_loc) = par.list[start.0].as_plain();
                let loc = Location {
                    begin: text_loc.begin + start.1 + delim_len,
                    end: text_loc.begin + end.1,
                };
                ct = vec![I::Plain(text[start.1 + delim_len..end.1].to_string(), loc)];

                text.replace_range(..end.1 + delim_len, "");
                text_loc.begin = text_loc.begin + end.1 + delim_len;

                end_offset += 1;
            } else {
                ins_pos = end.0 + 1;

                let (text, text_loc) = par.list[start.0].as_plain();
                let loc = Location {
                    begin: text_loc.begin + start.1 + delim_len,
                    end: text_loc.begin + end.1,
                };
                ct = vec![I::Plain(text[start.1 + delim_len..end.1].to_string(), loc)];

                if text.len() == end.1 + delim_len {
                    text.truncate(start.1);
                    text_loc.end = text_loc.begin + text.len();
                    end_offset += 1;
                } else {
                    let loc = Location {
                        begin: text_loc.begin + end.1 + delim_len,
                        end: text_loc.end,
                    };
                    let new = I::Plain(text[end.1 + delim_len..].to_string(), loc);

                    text.truncate(start.1);
                    text_loc.end = text_loc.begin + text.len();
                    par.list.push(new);

                    end_offset += 2;
                }
            }

            if par.open_brackets.len() > end_idx {
                let removed_len = end.1 + delim_len - 1;

                for el in &mut par.open_brackets[end_idx + 1..] {
                    match el {
                        Entity::Emph(_, _, pos, _) => {
                            if pos.0 > end.0 {
                                break;
                            }

                            pos.1 -= removed_len;
                        }

                        _ => (),
                    }
                }
            }
        } else {
            let seg_ends_at_plain_end
                = par.list[end.0].as_plain().0.len() == end.1 + delim_len;
            let end_idx_2 = if seg_ends_at_plain_end {
                end.0 + 1
            } else {
                end.0
            };

            ct = clip_segment(&mut par.list, *start, delim_len, Some(end_idx_2));
            end_offset -= end_idx_2 - start.0;
            ins_pos = if start.1 == 0 { start.0 } else { start.0 + 1 };

            if seg_ends_at_plain_end {
                ct.last_mut().unwrap().as_plain().0.truncate(end.1 - 1);
            } else {
                let (text, text_loc) = par.list[ins_pos].as_plain();
                let loc = Location {
                    begin: text_loc.begin,
                    end: text_loc.begin + end.1 - 1,
                };
                ct.push(I::Plain(text[..end.1 - 1].to_string(), loc));

                text.replace_range(..end.1 - 1 + delim_len, "");
                text_loc.begin = text_loc.begin + end.1 - 1 + delim_len;

                if par.open_brackets.len() > end_idx_2 {
                    let removed_len = end.1 - 1 + delim_len;

                    for el in &mut par.open_brackets[end_idx_2 + 1..] {
                        match el {
                            Entity::Emph(_, _, pos, _) => {
                                if pos.0 > end.0 {
                                    break;
                                }

                                pos.1 -= removed_len;
                            }

                            _ => (),
                        }
                    }
                }
            }
        }

        if delim_len == 1 {
            par.list.insert(ins_pos, I::Emph(ct));
        } else {
            let mut new = I::Strong(ct);
            let mut l = delim_len - 2;

            while l >= 2 {
                new = I::Strong(vec![new]);
                l -= 2;
            }

            if l == 1 {
                new = I::Emph(vec![new]);
            }

            par.list.insert(ins_pos, new);
        }

        lowest_start = Some(start_idx);
    }

    if matches!(par.list.last(), Some(I::Plain(..))) {
        if let Some(I::Plain(plain, loc)) = par.list.pop() {
            assert!(par.plain.is_empty());
            par.plain_begin = loc.begin;
            par.plain = plain;
        } else {
            unreachable!();
        }
    }

    if let Some(idx) = lowest_start {
        let idx = idx + 1;
        let mut i = 0;
        par.open_brackets.retain(|x| {
            i += 1;
            i < idx || !matches!(x, Entity::Emph(..))
        })
    } else {
        unreachable!("The for loop should have been run at least once");
    }
}

fn link_label(data: &mut impl ParserData) -> Option<String> {
    if !data.looking_at('[') {
        return None;
    }

    let mut label = String::new();
    let mut data = Transaction::new(data);

    loop {
        match data.next()? {
            '[' => return None,

            ']' => {
                data.advance();
                data.commit();
                return Some(label);
            }

            '\\' => {
                let ch = data.next()?;
                if ch != '[' && ch != ']' {
                    label.push('\\');
                }
                label.push(ch);
            }

            ch => label.push(ch),
        }
    }
}

pub fn linkdef(data: &mut impl ParserData) -> Option<Block> {
    let mut data = Transaction::new(data);
    log!(d, data, "link def", "begin");

    let label = link_label(&mut data)?;
    if label.trim().is_empty() {
        return None;
    }

    let (url, title) = linkdef_arg(&mut data)?;

    log!(d, data, "link def", "end");
    data.commit();
    Some(Block::LinkDef(label, url, title))
}

fn linkdef_arg(data: &mut impl ParserData) -> Option<(String, String)> {
    if !data.looking_at(':') {
        return None;
    }

    data.advance();
    data.skip_all(LINE_WS);
    if data.skip_newline() {
        data.skip_all(LINE_WS);

        if data.looking_at(NL_CR) {
            return None;
        }
    }

    let url = link_arg_url(data)?;

    let space_seen = data.skip_all(LINE_WS) > 0;

    if data.skip_newline() {
        let pos = data.pos();
        data.skip_all(LINE_WS);

        let title = link_arg_title(data)?;

        if data.skip_newline() || data.peek().is_none() {
            Some((url, title))
        } else {
            data.reset(pos).unwrap();
            Some((url, String::with_capacity(0)))
        }
    } else {
        let title = if space_seen {
            link_arg_title(data)?
        } else {
            String::with_capacity(0)
        };

        if data.skip_newline() || data.peek().is_none() {
            Some((url, title))
        } else {
            None
        }
    }
}
