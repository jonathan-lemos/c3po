#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedPosition<TLexeme> {
    lexemes: Vec<TLexeme>,
    pos: usize
}

impl<TLexeme> OwnedPosition<TLexeme> {
    pub fn new<I: IntoIterator<Item=TLexeme>>(lexemes: I) -> Self {
        Self {
            lexemes: lexemes.into_iter().collect(),
            pos: 0
        }
    }

    pub fn lexemes(&self) -> &[TLexeme] {
        &self.lexemes
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}
