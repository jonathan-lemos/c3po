use super::refcursor::RefCursor;
use super::super::cursor::Cursor;

impl<TLexeme: Clone> Iterator for RefCursor<TLexeme> {
    type Item = TLexeme;

    fn next(&mut self, )
}