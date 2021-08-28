/// A Cursor that owns the lexemes it points to.
/// 
/// This is much slower than RefCursor for large input sequences since it has to clone the lexemes for every `next()` call.
///
/// This Cursor's `Eq` and `PartialOrd` implementations compare the lexeme *values* for equality as well as the positions of the two cursors.
/// This is slow, but necessary since no two OwnedCursors share memory, making this the only sensible equality check.
///
#[derive(Debug, Clone)]
pub struct OwnedCursor<TLexeme: Clone> {
    pub(super) lexemes: Vec<TLexeme>,
    pub(super) pos: usize
}

impl<TLexeme: Clone> OwnedCursor<TLexeme> {
    pub fn new<I: IntoIterator<Item=TLexeme>>(lexemes: I) -> Self {
        Self {
            lexemes: lexemes.into_iter().collect(),
            pos: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::cursor::Cursor;

    #[test]
    fn new_ownedcursor_gets_first_char() {
        let cursor: OwnedCursor<char> = From::from("ab");
        let char = *cursor.current();

        assert_eq!(char, 'a')
    }
}
