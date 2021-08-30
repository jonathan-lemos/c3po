use super::cursor::Cursor;
use core::ops::Add;

impl<TLexeme> Add<usize> for Cursor<'_, TLexeme> {
    type Output = Option<Self>;

    /// Increments the Cursor by `n` lexemes
    /// 
    /// Returns `None` if the new position of the cursor is out-of-bounds.
    fn add(self, n: usize) -> Self::Output {
        let new_pos = self.pos + n;
        if n >= self.source.len() {
            None
        } else {
            Some(Self {
                source: self.source,
                pos: new_pos
            })
        }
    }
}

impl<'a, TLexeme> Add<usize> for &'a Cursor<'_, TLexeme> {
    type Output = Option<Cursor<'a, TLexeme>>;

    /// Increments the Cursor by `n` lexemes
    /// 
    /// Returns `None` if the new position of the cursor is out-of-bounds.
    fn add(self, n: usize) -> Self::Output {
        let new_pos = self.pos + n;
        if n >= self.source.len() {
            None
        } else {
            Some(Cursor {
                source: self.source,
                pos: new_pos
            })
        }
    }
}
