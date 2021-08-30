use super::cursor::Cursor;
use crate::immut_iter::immut_iterable::ImmutableIterable;

impl<TLexeme> ImmutableIterable for Cursor<'_, TLexeme> {
    fn next_immut(&self) -> Option<Self> {
        self + 1
    }
}
