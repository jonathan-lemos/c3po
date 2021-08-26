use super::refcursor::RefCursor;
use std::cmp::Ordering;
use std::ptr;

impl<TLexeme> PartialOrd for RefCursor<'_, TLexeme> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !ptr::eq(self.source, other.source) {
            None
        } else {
            PartialOrd::partial_cmp(&self.pos, &other.pos)
        }
    }
}
