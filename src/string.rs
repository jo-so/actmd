use std::str::CharIndices;

use super::{
    ParserData,
    ParserSettings,
    Position,
};

pub struct StringData<'a> {
    settings: ParserSettings,

    data: &'a str,
    slice: CharIndices<'a>,
    start: usize,
    pos: usize,
    peek: Option<char>,
}

impl<'a> StringData<'a> {
    #[must_use]
    pub fn new(
        data: &'a str, settings: ParserSettings
    ) -> Self {
        let mut val = Self {
            settings,

            data,
            slice: data.char_indices(),
            start: 0,
            pos: 0,
            peek: None,
        };

        if let Some((pos, ch)) = val.slice.next() {
            // Sanitize [insecure
            // characters](https://spec.commonmark.org/0.29/#insecure-characters)
            val.peek = Some(if ch == '\0' { '\u{fffd}' } else { ch });
            val.pos = pos;
        }

        val
    }
}

impl ParserData for StringData<'_> {
    fn settings(&self) -> ParserSettings {
        self.settings
    }

    fn peek(&self) -> Option<char> {
        self.peek
    }

    fn advance(&mut self) {
        if self.peek().is_none() {
            return;
        }

        match self.slice.next() {
            None => {
                self.peek = None;
                self.pos += 1;
            }

            Some((pos, ch)) => {
                // Sanitize [insecure
                // characters](https://spec.commonmark.org/0.29/#insecure-characters)
                self.peek = Some(if ch == '\0' { '\u{fffd}' } else { ch });
                self.pos = pos;
            }
        }
    }

    fn pos(&self) -> Position  {
        self.start.checked_add(self.pos).unwrap()
    }

    fn reset(&mut self, pos: Position) -> Result<(), ()> {
        if pos > self.data.len() {
            return Err(());
        }

        self.slice = self.data[pos..].char_indices();
        self.peek = self.slice.next().map(|(_, x)| x);
        self.start = pos;
        self.pos = 0;

        Ok(())
    }
}

impl<'a> From<&'a str> for StringData<'a> {
    fn from(input: &'a str) -> Self {
        Self::new(input, Default::default())
    }
}

impl<'a> From<&'a String> for StringData<'a> {
    fn from(input: &'a String) -> Self {
        Self::new(input.as_str(), Default::default())
    }
}

// TODO:
/*
pub struct StringTransaction<'a, 'b: 'a> {
    inner: &'a mut StringInput<'b>,
    start_pos: usize,
}

impl Drop for StringTransaction<'_, '_> {
    fn drop(&mut self) {
        self.inner.reset(self.start_pos);
    }
}

impl InputTransaction for StringTransaction<'_, '_> {
    fn commit(self: Box<Self>) {
        // don't call drop
        mem::forget(self);
    }
}

impl ParserInput for StringTransaction<'_, '_> {
    fn options(&self) -> ParserSettings {
        self.inner.options()
    }

    fn has_opt(&self, opt: ParserSettings) -> bool {
        self.inner.has_opt(opt)
    }

    fn peek(&self) -> Option<char> {
        self.inner.peek()
    }

    fn advance(&mut self) {
        self.inner.advance()
    }

    fn pos(&self) -> usize {
        self.inner.pos()
    }

    fn as_transaction<'a, 'b: 'a>(&'b mut self) -> Box<dyn InputTransaction + 'a> {
        Box::new(StringTransaction {
            start_pos: self.inner.pos(),
            inner: self.inner,
        })
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Tools;

    #[test]
    fn empty_input() {
        let val = StringData::from("");

        assert_eq!((0, None), val.pos_peek());
    }

    #[test]
    fn simple_test() {
        let mut val = StringData::from("abc");

        assert_eq!((0, Some('a')), val.pos_peek());
        val.advance();
        assert_eq!((1, Some('b')), val.pos_peek());
        val.advance();
        assert_eq!((2, Some('c')), val.pos_peek());
        val.advance();
        assert_eq!((3, None), val.pos_peek());
    }

    #[test]
    fn utf_char() {
        let mut val = StringData::from("xÄy");
        assert_eq!((0, Some('x')), val.pos_peek());
        val.advance();
        assert_eq!((1, Some('Ä')), val.pos_peek());
        val.advance();
        assert_eq!((3, Some('y')), val.pos_peek());
    }

    #[test]
    fn advance_after_end() {
        let mut val = StringData::from("x");

        assert_eq!((0, Some('x')), val.pos_peek());
        val.advance();
        assert_eq!((1, None), val.pos_peek());

        val.advance();
        assert_eq!((1, None), val.pos_peek());
    }
}
