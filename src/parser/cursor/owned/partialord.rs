use std::cmp::Ordering;
use super::ownedcursor::OwnedCursor;

impl<TLexeme: Clone + PartialEq> PartialOrd for OwnedCursor<TLexeme> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lexemes != other.lexemes {
            None
        } else {
            PartialOrd::partial_cmp(&self.pos, &other.pos)
        }
    }
}
