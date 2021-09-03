use super::cursor::Cursor;

impl<'a, TLexeme> Clone for Cursor<'a, TLexeme> {
    fn clone(&self) -> Self {
       *self 
    }
}

impl<'a, TLexeme> Copy for Cursor<'a, TLexeme> {}
