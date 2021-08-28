use super::refcursor::RefCursor;
use std::ptr;

impl<TLexeme> PartialEq for RefCursor<'_, TLexeme> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.source, other.source) && self.pos == other.pos
    }
}

impl<TLexeme> Eq for RefCursor<'_, TLexeme> {}
