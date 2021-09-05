use super::cursor::Cursor;
use std::ptr;

impl PartialEq for Cursor<'_> {
    /// Returns `true` if the input slices are the same *pointer* and the positions of both Cursors are the same.
    /// 
    /// This does not check that the input slices are value equal.
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.sequence, other.sequence) && self.pos_chars == other.pos_chars
    }
}

impl Eq for Cursor<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_is_equal() {
        let c1 = Cursor::new("abc").unwrap();
        let c2 = c1.clone();
        assert_eq!(c1, c2)
    }

    #[test]
    fn cursor_with_same_origin_is_equal() {
        let s = "abc";
        let c1 = Cursor::new(s).unwrap();
        let c2 = Cursor::new(s).unwrap();
        assert_eq!(c1, c2)
    }

    #[test]
    fn cursor_with_equivalent_origin_is_ne() {
        let s1 = "vec".to_owned();
        let s2 = "vec".to_owned();
        let c1 = Cursor::new(&s1).unwrap();
        let c2 = Cursor::new(&s2).unwrap();
        assert_ne!(c1, c2)
    }

    #[test]
    fn cursor_with_different_pos_is_ne() {
        let c1 = Cursor::new("abc").unwrap();
        let c2 = (c1 + 1).unwrap();
        assert_ne!(c1, c2)
    }
}
