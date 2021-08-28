use super::super::cursor::Cursor;
use super::cursoriterator::CursorIterator;

impl<TLexeme: Clone, TCursor: Cursor<Lexeme = TLexeme>> Iterator
    for CursorIterator<TLexeme, TCursor>
{
    type Item = TCursor;

    fn next(&mut self) -> Option<Self::Item> {
        let (next, prev) = match &self.cursor {
            None => (None, None),
            Some(c) => {
                (c.next_immut(), Some(c.clone()))
            }
        };
        self.cursor = next;
        prev
    }
}
