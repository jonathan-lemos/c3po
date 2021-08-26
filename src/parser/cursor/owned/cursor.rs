use super::super::cursor::Cursor;
use super::ownedcursor::OwnedCursor;

impl<TLexeme: Clone + PartialEq + Send + Sync> Cursor for OwnedCursor<TLexeme> {
    type Lexeme = TLexeme;

    fn pos(&self) -> usize {
        self.pos
    }

    fn source(&self) -> &[Self::Lexeme] {
        &self.lexemes
    }
}
