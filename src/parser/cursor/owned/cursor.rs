use super::super::cursor::Cursor;
use super::ownedcursor::OwnedCursor;

impl<TLexeme: Clone + Send + Sync> Cursor for OwnedCursor<TLexeme> {
    type Lexeme = TLexeme;

    fn next_immut(&self) -> Option<Self> {
        if self.lexemes.len() <= self.pos {
            None
        } else {
            Some(OwnedCursor {
                lexemes: self.lexemes.clone(),
                pos: self.pos + 1
            })
        }
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn source(&self) -> &[Self::Lexeme] {
        &self.lexemes
    }
}
