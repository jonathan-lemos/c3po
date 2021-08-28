use super::super::cursor::Cursor;
use super::refcursor::RefCursor;

impl<TLexeme: Clone + Send + Sync> Cursor for RefCursor<'_, TLexeme> {
    type Lexeme = TLexeme;

    fn next_immut(&self) -> Option<Self> {
        if self.source.len() <= self.pos {
            None
        } else {
            Some(RefCursor {
                source: self.source,
                pos: self.pos + 1
            })
        }
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn source(&self) -> &[Self::Lexeme] {
        self.source
    }
}
