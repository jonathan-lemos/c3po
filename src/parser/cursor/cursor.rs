use std::cmp::min;
use std::ops::Add;

pub trait Cursor<TLexeme>: Sized + Eq + PartialOrd + Clone + Add<usize, Output = Self> {
    fn input(&self) -> &[TLexeme] {
        let pos = min(self.pos(), self.source().len());
        &self.source()[pos..]
    }

    fn next(self) -> Self {
        self + 1
    }

    fn pos(&self) -> usize;

    fn source(&self) -> &[TLexeme];
}
