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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::owned::ownedcursor::OwnedCursor;

    #[test]
    fn cursoriterator_iterates() {
        let chars = vec!['a', 'b', 'c'];
        let cursor = OwnedCursor::new(chars.clone().into_iter());
        let it = cursor.iter();

        let result: Vec<char> = it.map(|c| *c.current()).collect();
        assert_eq!(chars, result);
    }
}
