#[derive(Debug, Clone, Copy)]
pub struct RefPosition<'a, TLexeme> {
    pub(super) input: &'a [TLexeme],
    pub(super) source: &'a [TLexeme],
    pub(super) pos: usize
}

impl<'a, TLexeme> RefPosition<'a, TLexeme> {
    pub fn new(lexemes: &'a [TLexeme]) -> Self {
        Self {
            input: lexemes,
            source: lexemes,
            pos: 0
        }
    }
}
