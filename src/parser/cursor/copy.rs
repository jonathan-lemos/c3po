use super::cursor::Cursor;

impl<'a, TLexeme> Clone for Cursor<'a, TLexeme> {
    fn clone(&self) -> Self {
        Cursor {
            source: self.source,
            pos: self.pos
        }
    }
}

impl<'a, TLexeme> Copy for Cursor<'a, TLexeme> {}
