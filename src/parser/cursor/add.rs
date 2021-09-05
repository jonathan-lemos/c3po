use super::cursor::Cursor;
use crate::immut_iter::immut_iterable::ImmutableIterable;
use core::ops::Add;

impl Add<usize> for Cursor<'_> {
    type Output = Option<Self>;

    /// Increments the Cursor by `n` lexemes
    /// 
    /// Returns `None` if the new position of the cursor is out-of-bounds.
    fn add(self, n: usize) -> Self::Output {
        (0..n).fold(Some(self), |a, _| a.and_then(|c| c.next_immut()))
    }
}

impl<'a> Add<usize> for &Cursor<'a> {
    type Output = Option<Cursor<'a>>;

    /// Increments the Cursor by `n` lexemes
    /// 
    /// Returns `None` if the new position of the cursor is out-of-bounds.
    fn add(self, n: usize) -> Self::Output {
        *self + n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_add_1_goes_to_next_char() {
        let c = Cursor::new("0123").unwrap();

        assert_eq!((c + 1).unwrap().current(), '1')
    }

    #[test]
    fn cursor_add_2_goes_to_third_char() {
        let c = Cursor::new("0123").unwrap();

        assert_eq!((c + 2).unwrap().current(), '2')
    }

    #[test]
    fn cursor_overflow_is_none() {
        let c = Cursor::new("0123").unwrap();

        assert!((c + 4).is_none());
        assert!((c + 5).is_none());
    }
}
