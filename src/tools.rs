use super::{
    ParserData,
    ParserSettings,
};

pub trait Pattern {
    fn matches(&self, ch: char) -> bool;
}

impl Pattern for char {
    fn matches(&self, ch: char) -> bool {
        ch == *self
    }
}

impl Pattern for [char; 2] {
    fn matches(&self, ch: char) -> bool {
        self[0] == ch || self[1] == ch
    }
}

impl Pattern for &[char] {
    fn matches(&self, ch: char) -> bool {
        self.iter().any(|c| *c == ch)
    }
}

impl Pattern for &str {
    fn matches(&self, ch: char) -> bool {
        self.chars().any(|c| c == ch)
    }
}

impl<F: Fn(char) -> bool> Pattern for F {
    fn matches(&self, ch: char) -> bool {
        self(ch)
    }
}

pub trait Tools {
    /// Tests if a Setting *sett* is set.
    fn has_setting(&self, sett: ParserSettings) -> bool;

    fn looking_at(&self, pat: impl Pattern) -> bool;
    fn expect_char(&mut self, ch: char);
    fn next(&mut self) -> Option<char>;
    fn skip(&mut self, pat: impl Pattern) -> bool;
    fn skip_newline(&mut self) -> bool;
    fn skip_all(&mut self, pat: impl Pattern) -> usize;
    fn copy_all(&mut self, buf: &mut String, pat: impl Pattern) -> usize;
    fn copy_until(&mut self, buf: &mut String, pat: impl Pattern) -> bool;
    fn copy_until_seq(&mut self, buf: &mut String, stop: &str) -> bool;
    fn copy_until_match(&mut self, buf: &mut String, open: char, close: char) -> bool;

    #[cfg(any(test, log))]
    fn pos_peek(&self) -> (usize, Option<char>);
}

impl<T: ParserData + ?Sized> Tools for T {
    fn has_setting(&self, sett: ParserSettings) -> bool {
        self.settings().contains(sett)
    }

    fn looking_at(&self, pat: impl Pattern) -> bool {
        self.peek().map_or(false, |c| pat.matches(c))
    }

    fn expect_char(&mut self, ch: char) {
        #[cfg(debug_assertions)]
        {
            let peek = self.peek();

            let pos = self.pos();
            debug_assert_eq!(
                Some(ch), peek,
                "Expected '{ch}' at {pos}, but got {peek:?}"
            );
        }

        self.advance();
    }

    fn next(&mut self) -> Option<char> {
        self.advance();
        self.peek()
    }

    fn skip(&mut self, pat: impl Pattern) -> bool {
        if self.looking_at(pat) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_newline(&mut self) -> bool {
        if self.skip('\r') {
            self.skip('\n');
            true
        } else {
            self.skip('\n')
        }
    }

    fn skip_all(&mut self, pat: impl Pattern) -> usize {
        let mut cnt = 0;
        let pat = &pat;

        while let Some(c) = self.peek() {
            if !pat.matches(c) {
                break;
            }

            self.advance();
            cnt += 1;
        }
        cnt
    }

    fn copy_all(&mut self, buf: &mut String, pat: impl Pattern) -> usize {
        let mut cnt = 0;
        // let pat = &pat;

        while let Some(c) = self.peek() {
            if !pat.matches(c) {
                break;
            }

            self.advance();
            buf.push(c);
            cnt += 1;
        }

        cnt
    }

    fn copy_until(&mut self, buf: &mut String, pat: impl Pattern) -> bool {
        let pat = &pat;

        while let Some(c) = self.peek() {
            self.advance();
            buf.push(c);

            if pat.matches(c) {
                return true;
            }
        }

        false
    }

    fn copy_until_seq(&mut self, buf: &mut String, stop: &str) -> bool {
        assert!(!stop.is_empty());

        loop {
            let mut stop_it = stop.chars();
            // copy everything up to the first char from *stop*
            if !self.copy_until(buf, stop_it.next().unwrap()) {
                return false;
            }

            // check if the rest of *stop* matches
            loop {
                match (stop_it.next(), self.peek()) {
                    (None, _) => {
                        // self.advance();
                        return true;
                    },
                    (_, None) => return false,

                    (Some(c1), Some(c2)) => {
                        if c1 == c2 {
                            buf.push(c2);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn copy_until_match(&mut self, buf: &mut String, open: char, close: char) -> bool {
        let start_len = buf.len();

        let mut open_cnt = 1;
        while let Some(ch) = self.peek() {
            self.advance();

            match ch {
                x if x == open => {
                    buf.push(ch);
                    open_cnt += 1;
                }

                x if x == close => {
                    buf.push(ch);
                    open_cnt -= 1;

                    if open_cnt == 0 {
                        return true;
                    }
                }

                '\\' => {
                    if let Some(c) = self.peek() {
                        if c == '\\' || c == open || c == close {
                            self.advance();
                            buf.push(c);
                        } else {
                            buf.push('\\');
                        }
                    } else {
                        break;
                    }
                }

                c => buf.push(c),
            }
        }

        buf.truncate(start_len);
        false
    }

    #[cfg(any(test, log))]
    fn pos_peek(&self) -> (usize, Option<char>) {
        (self.pos(), self.peek())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::StringData;

    #[test]
    fn looking_at_char() {
        let mut data = StringData::from("abc");

        assert!(data.looking_at('a'));
        data.advance();
        assert!(!data.looking_at('a'));
    }

    #[test]
    fn looking_at_char_slice() {
        let mut data = StringData::from("abc");

        assert!(data.looking_at(&['a', 'b'][..]));
        data.advance();
        assert!(data.looking_at(&['a', 'b'][..]));
        assert!(!data.looking_at(&['0', '1'][..]));
    }

    #[test]
    fn looking_at_fn() {
        let data = StringData::from("abc");

        assert!(data.looking_at(char::is_alphabetic));
        assert!(data.looking_at(|c| c == 'a'));

        assert!(!data.looking_at(char::is_control));
        assert!(!data.looking_at(|c| c != 'a'));
    }

    #[test]
    #[should_panic(expected = "Expected '0' at 1, but got Some('b')")]
    fn expect_char() {
        let mut data = StringData::from("abc");

        data.expect_char('a');
        data.expect_char('0');
    }

    // TODO: Tests für Skip

    #[test]
    fn skip_newline_nothing() {
        let mut data = StringData::from("a\nbc");
        assert!(!data.skip_newline());
        assert_eq!((0, Some('a')), data.pos_peek());
    }

    #[test]
    fn skip_newline_empty() {
        let mut data = StringData::from("");
        assert!(!data.skip_newline());
        assert_eq!((0, None), data.pos_peek());
    }

    #[test]
    fn skip_newline_cr() {
        let mut data = StringData::from("\rabc");
        assert!(data.skip_newline());
        assert_eq!((1, Some('a')), data.pos_peek());
    }

    #[test]
    fn skip_newline_nl() {
        let mut data = StringData::from("\nabc");
        assert!(data.skip_newline());
        assert_eq!((1, Some('a')), data.pos_peek());
    }

    #[test]
    fn skip_newline_crnl() {
        let mut data = StringData::from("\r\nabc");
        assert!(data.skip_newline());
        assert_eq!((2, Some('a')), data.pos_peek());
    }

    #[test]
    fn skip_newline_end_cr() {
        let mut data = StringData::from("\r");
        assert!(data.skip_newline());
        assert_eq!((1, None), data.pos_peek());
    }

    #[test]
    fn skip_newline_end_nl() {
        let mut data = StringData::from("\n");
        assert!(data.skip_newline());
        assert_eq!((1, None), data.pos_peek());
    }

    #[test]
    fn skip_newline_end_crnl() {
        let mut data = StringData::from("\r\n");
        assert!(data.skip_newline());
        assert_eq!((2, None), data.pos_peek());
    }

    #[test]
    fn skip_newline_once_cr() {
        let mut data = StringData::from("\r\rabc");
        assert!(data.skip_newline());
        assert_eq!((1, Some('\r')), data.pos_peek());
    }

    #[test]
    fn skip_newline_once_nl() {
        let mut data = StringData::from("\n\nabc");
        assert!(data.skip_newline());
        assert_eq!((1, Some('\n')), data.pos_peek());
    }

    #[test]
    fn skip_newline_once_crnl() {
        let mut data = StringData::from("\r\n\r\nabc");
        assert!(data.skip_newline());
        assert_eq!((2, Some('\r')), data.pos_peek());
    }

    #[test]
    fn skip_all_empty_input() {
        let mut data = StringData::from("");
        let pre_state = data.pos_peek();

        assert_eq!(0, data.skip_all('0'));
        assert_eq!(pre_state, data.pos_peek());

        assert_eq!(0, data.skip_all(&['0', '1'][..]));
        assert_eq!(pre_state, data.pos_peek());

        assert_eq!(0, data.skip_all(char::is_control));
        assert_eq!(pre_state, data.pos_peek());

        assert_eq!(0, data.skip_all(|c| c != 'a'));
        assert_eq!(pre_state, data.pos_peek());
    }

    #[test]
    fn skip_all_nothing() {
        let mut data = StringData::from("ababababc");
        let pre_state = data.pos_peek();

        assert_eq!(0, data.skip_all('0'));
        assert_eq!(pre_state, data.pos_peek());

        assert_eq!(0, data.skip_all(&['0', '1'][..]));
        assert_eq!(pre_state, data.pos_peek());

        assert_eq!(0, data.skip_all(char::is_control));
        assert_eq!(pre_state, data.pos_peek());

        assert_eq!(0, data.skip_all(|c| c != 'a'));
        assert_eq!(pre_state, data.pos_peek());
    }

    #[test]
    fn skip_all_everything() {
        macro_rules! test {
            ($inp:literal, $cnt:literal, $arg:expr) => (
                let mut data = StringData::from($inp);
                assert_eq!($cnt, data.skip_all($arg));
                assert_eq!(($cnt, None), data.pos_peek());
            );
        }

        test!("aaa", 3, 'a');
        test!("abaaabbab", 9, &['a', 'b'][..]);
        test!("abaaabbab", 9, char::is_alphabetic);
        test!("abaaabbab", 9, |c: char| c.is_ascii_alphabetic());
    }

    #[test]
    fn skip_all_something() {
        macro_rules! test {
            ($inp:literal, $cnt:literal, $arg:expr) => (
                let mut data = StringData::from($inp);
                assert_eq!($cnt, data.skip_all($arg));
                assert_eq!(($cnt, Some('+')), data.pos_peek());
            );
        }

        test!("aaa+", 3, 'a');
        test!("abaaabbab+", 9, &['a', 'b'][..]);
        test!("abaaabbab+", 9, char::is_alphabetic);
        test!("abaaabbab+", 9, |c: char| c.is_ascii_alphabetic());
    }

    #[test]
    fn copy_all_empty_input() {
        let mut data = StringData::from("");
        let mut buf = String::new();
        let pre_state = data.pos_peek();

        macro_rules! test {
            ($val:expr) => (
                assert_eq!(0, data.copy_all(&mut buf, $val));
                assert_eq!(pre_state, data.pos_peek());
                assert!(buf.is_empty());
            );
        }

        test!('0');
        test!(&['0', '1'][..]);
        test!(char::is_control);
        test!(|c| c != 'a');
    }

    #[test]
    fn copy_all_nothing() {
        let mut data = StringData::from("ababababc");
        let mut buf = String::new();
        let pre_state = data.pos_peek();

        macro_rules! test {
            ($val:expr) => (
                assert_eq!(0, data.copy_all(&mut buf, $val));
                assert_eq!(pre_state, data.pos_peek());
                assert!(buf.is_empty());
            );
        }

        test!('0');
        test!(&['0', '1'][..]);
        test!(char::is_control);
        test!(|c| c != 'a');
    }

    #[test]
    fn copy_all_everything() {
        macro_rules! test {
            ($inp:literal, $cnt:literal, $arg:expr) => (
                let mut data = StringData::from($inp);
                let mut buf = String::new();
                assert_eq!($cnt, data.copy_all(&mut buf, $arg));
                assert_eq!(($cnt, None), data.pos_peek());
                assert_eq!($inp, buf);
            );
        }

        test!("aaa", 3, 'a');
        test!("abaaabbab", 9, &['a', 'b'][..]);
        test!("abaaabbab", 9, char::is_alphabetic);
        test!("abaaabbab", 9, |c: char| c.is_ascii_alphabetic());
    }

    #[test]
    fn copy_all_something() {
        macro_rules! test {
            ($inp:literal, $cnt:literal, $arg:expr) => (
                let mut data = StringData::from($inp);
                let mut buf = String::new();
                assert_eq!($cnt, data.copy_all(&mut buf, $arg));
                assert_eq!(($cnt, Some('+')), data.pos_peek());
                assert_eq!($inp[0..$cnt], buf);
            );
        }

        test!("aaa+", 3, 'a');
        test!("abaaabbab+", 9, &['a', 'b'][..]);
        test!("abaaabbab+", 9, char::is_alphabetic);
        test!("abaaabbab+", 9, |c: char| c.is_ascii_alphabetic());
    }

    #[test]
    fn copy_until_seq() {
        macro_rules! test {
            ($inp:literal, $cnt:literal, $next:expr, $arg:expr) => (
                let mut data = StringData::from($inp);
                let mut buf = String::new();
                assert_eq!($cnt > 0, data.copy_until_seq(&mut buf, $arg));
                assert_eq!(($cnt, $inp[$cnt..].chars().next()), data.pos_peek());
                assert_eq!($inp[0..$cnt], buf);
            );
        }

        // test!("", 0, None, "++");
        test!("++", 2, None, "++");
        // test!("text++", 6, None, "++");
        // test!("text++#", 6, Some('#'), "++");
    }
}
