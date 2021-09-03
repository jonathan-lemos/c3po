use super::cursor::Cursor;
use std::ptr;

impl<TLexeme> PartialEq for Cursor<'_, TLexeme> {
    /// Returns `true` if the input slices are the same *pointer* and the positions of both Cursors are the same.
    /// 
    /// This does not check that the input slices are value equal.
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.source, other.source) && self.pos == other.pos
    }
}

impl<TLexeme> Eq for Cursor<'_, TLexeme> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_is_equal() {
        let vec = vec![1];
        let c1 = Cursor::new(&vec).unwrap();
        let c2 = c1.clone();
        assert_eq!(c1, c2)
    }

    #[test]
    fn cursor_with_same_origin_is_equal() {
        let vec = vec![1];
        let c1 = Cursor::new(&vec).unwrap();
        let c2 = Cursor::new(&vec).unwrap();
        assert_eq!(c1, c2)
    }

    #[test]
    fn cursor_with_equivalent_origin_is_ne() {
        let vec = vec![1];
        let vec2 = vec![1];
        let c1 = Cursor::new(&vec).unwrap();
        let c2 = Cursor::new(&vec2).unwrap();
        assert_ne!(c1, c2)
    }

    #[test]
    fn cursor_with_different_pos_is_ne() {
        let vec = vec![1, 2];
        let c1 = Cursor::new(&vec).unwrap();
        let c2 = (c1 + 1).unwrap();
        assert_ne!(c1, c2)
    }
}
