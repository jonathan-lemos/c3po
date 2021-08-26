use std::cmp::Ordering;
use std::ptr;
use super::position::Position;

impl<TLexeme> PartialOrd for Position<'_, TLexeme> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !ptr::eq(self.source, other.source) {
            None
        } else {
            PartialOrd::partial_cmp(&self.pos, &other.pos)
        }
    }
}
