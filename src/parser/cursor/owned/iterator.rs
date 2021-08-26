use super::ownedcursor::OwnedCursor;

impl<TLexeme: Clone + PartialEq + Send + Sync> Iterator for OwnedCursor<TLexeme> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lexemes.len() <= self.pos {
            None
        } else {
            Some(OwnedCursor {
                lexemes: self.lexemes.clone(),
                pos: self.pos + 1
            })
        }
    }
}
