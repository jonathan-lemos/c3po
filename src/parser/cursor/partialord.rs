use super::cursor::Cursor;
use std::cmp::Ordering;
use std::ptr;

impl<TLexeme> PartialOrd for Cursor<'_, TLexeme> {
    /// Compares the two `pos()` values of the two Cursors.
    /// 
    /// Returns `None` if the two input slices are different *pointers*.
    /// This does not check the two slices for value equality.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !ptr::eq(self.source, other.source) {
            None
        } else {
            PartialOrd::partial_cmp(&self.pos, &other.pos)
        }
    }
}
