use super::cursor::Cursor;
use core::ops::Add;

impl<TLexeme> Add<usize> for Cursor<'_, TLexeme> {
    type Output = Option<Self>;

    /// Increments the Cursor by `n` lexemes
    /// 
    /// Returns `None` if the new position of the cursor is out-of-bounds.
    fn add(self, n: usize) -> Self::Output {
        let new_pos = self.pos + n;
        if new_pos >= self.source.len() {
            None
        } else {
            Some(Self {
                source: self.source,
                pos: new_pos
            })
        }
    }
}

impl<'a, TLexeme> Add<usize> for &Cursor<'a, TLexeme> {
    type Output = Option<Cursor<'a, TLexeme>>;

    /// Increments the Cursor by `n` lexemes
    /// 
    /// Returns `None` if the new position of the cursor is out-of-bounds.
    fn add(self, n: usize) -> Self::Output {
        let new_pos = self.pos + n;
        if new_pos >= self.source.len() {
            None
        } else {
            Some(Cursor {
                source: self.source,
                pos: new_pos
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_add_1_goes_to_next_char() {
        let chars = "0123".chars().collect::<Vec<char>>();
        let c = Cursor::new(&chars).unwrap();

        assert_eq!((c + 1).unwrap().current(), &'1')
    }

    #[test]
    fn cursor_add_2_goes_to_third_char() {
        let chars = "0123".chars().collect::<Vec<char>>();
        let c = Cursor::new(&chars).unwrap();

        assert_eq!((c + 2).unwrap().current(), &'2')
    }

    #[test]
    fn cursor_overflow_is_none() {
        let chars = "0123".chars().collect::<Vec<char>>();
        let c = Cursor::new(&chars).unwrap();

        assert!((c + 4).is_none());
        assert!((c + 5).is_none());
    }
}
