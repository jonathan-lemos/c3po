use super::super::cursor::Cursor;

pub struct CursorIterator<TLexeme, TCursor: Cursor<Lexeme = TLexeme>> {
    pub(super) cursor: Option<TCursor>
}

impl<TLexeme, TCursor: Cursor<Lexeme = TLexeme>> CursorIterator<TLexeme, TCursor> {
    pub fn new(cursor: &TCursor) -> Self {
        CursorIterator {
            cursor: Some(cursor.clone())
        }
    }

    pub fn next_n(n: usize) -> Option<>
}
