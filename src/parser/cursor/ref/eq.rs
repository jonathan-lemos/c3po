use super::position::Position;
use std::ptr;

impl<TLexeme> PartialEq for Position<'_, TLexeme> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.input, other.input) && ptr::eq(self.source, other.source) && self.pos == other.pos
    }
}

impl<TLexeme> Eq for Position<'_, TLexeme> {}
