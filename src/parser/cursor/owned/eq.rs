use super::ownedcursor::OwnedCursor;

impl<TLexeme: Clone + PartialEq> PartialEq for OwnedCursor<TLexeme> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.lexemes == other.lexemes
    }
}

impl<TLexeme: Clone + Eq> Eq for OwnedCursor<TLexeme> {}
