use super::super::cursor::Cursor;
use super::cursoriterator::CursorIterator;

impl<TLexeme: Clone, TCursor: Cursor<Lexeme = TLexeme>> Iterator
    for CursorIterator<TLexeme, TCursor>
{
    type Item = TCursor;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.map(|c| {
            let current = c.current();
            self.cursor = c.next_immut();
            c
        })
    }
}
