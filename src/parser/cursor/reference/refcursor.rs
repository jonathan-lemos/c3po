/// A Cursor that references a slice of lexemes.

#[derive(Debug, Clone, Copy)]
pub struct RefCursor<'a, TLexeme> {
    pub(super) source: &'a [TLexeme],
    pub(super) pos: usize
}

impl<'a, TLexeme> RefCursor<'a, TLexeme> {
    pub fn new(lexemes: &'a [TLexeme]) -> Self {
        Self {
            source: lexemes,
            pos: 0
        }
    }
}
