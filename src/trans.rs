use super::{
    ParserData,
    ParserSettings,
    Position,
};

pub struct Transaction<'a, Data: ParserData>{
    inner: Option<&'a mut Data>,
    start: Position,
}

impl<'a, D: ParserData> Transaction<'a, D> {
    pub fn new(inner: &'a mut D) -> Self {
        Self {
            start: inner.pos(),
            inner: Some(inner),
        }
    }

    pub fn commit(mut self) -> &'a D {
        let inner = self.inner.take().unwrap();
        std::mem::forget(self);
        inner
    }

    fn inner(&self) -> &D {
        self.inner.as_ref().unwrap()
    }

    fn inner_mut(&mut self) -> &mut D {
        self.inner.as_mut().unwrap()
    }
}

impl<D: ParserData> ParserData for Transaction<'_, D> {
    fn settings(&self) -> ParserSettings {
        self.inner().settings()
    }

    fn peek(&self) -> Option<char> {
        self.inner().peek()
    }

    fn advance(&mut self) {
        self.inner_mut().advance();
    }

    fn pos(&self) -> Position {
        self.inner().pos()
    }

    fn reset(&mut self, pos: Position) -> Result<(), ()> {
        if pos < self.start {
            Err(())
        } else {
            self.inner_mut().reset(pos)
        }
    }
}

impl<D: ParserData> Drop for Transaction<'_, D> {
    fn drop(&mut self) {
        // TODO: Was soll mit self.inner.messages passieren?
        if self.inner.as_mut().unwrap().reset(self.start).is_err() {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{StringData, Tools};

    #[test]
    fn simple_commit() {
        let mut val = StringData::from("abc");
        let val = &mut val;

        assert_eq!((0, Some('a')), val.pos_peek());
        val.advance();

        assert_eq!((1, Some('b')), val.pos_peek());

        {
            let mut trans = Transaction::new(val);

            assert_eq!((1, Some('b')), trans.pos_peek());
            trans.advance();
            assert_eq!((2, Some('c')), trans.pos_peek());

            trans.commit();
        }

        assert_eq!((2, Some('c')), val.pos_peek());
    }

    #[test]
    fn explicit_rollback() {
        let mut val = StringData::from("abc");
        let val = &mut val;

        assert_eq!((0, Some('a')), val.pos_peek());
        val.advance();

        assert_eq!((1, Some('b')), val.pos_peek());

        {
            let mut trans = Transaction::new(val);

            assert_eq!((1, Some('b')), trans.pos_peek());
            trans.advance();
            assert_eq!((2, Some('c')), trans.pos_peek());

            drop(trans); // i.e. explicit rollback
        }

        assert_eq!((1, Some('b')), val.pos_peek());
    }

    #[test]
    fn implicit_rollback() {
        let mut val = StringData::from("abc");
        let val = &mut val;

        assert_eq!((0, Some('a')), val.pos_peek());
        val.advance();

        assert_eq!((1, Some('b')), val.pos_peek());

        {
            let mut trans = Transaction::new(val);

            assert_eq!((1, Some('b')), trans.pos_peek());
            trans.advance();
            assert_eq!((2, Some('c')), trans.pos_peek());

            // i.e. implicit rollback
        }

        assert_eq!((1, Some('b')), val.pos_peek());
    }
}
