use super::cursor::Cursor;
use std::ptr;

impl<TLexeme> PartialEq for Cursor<'_, TLexeme> {
    /// Returns `true` if the input slices are the same *pointer* and the positions of both Cursors are the same.
    /// 
    /// This does not check that the input slices are value equal.
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.source, other.source) && self.pos == other.pos
    }
}

impl<TLexeme> Eq for Cursor<'_, TLexeme> {}
