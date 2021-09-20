use super::cursor::Cursor;
use std::cmp::Ordering;
use std::ptr;

impl PartialOrd for Cursor<'_> {
    /// Compares the two `pos()` values of the two Cursors.
    ///
    /// Returns `None` if the two input slices are different *pointers*.
    /// This does not check the two slices for value equality.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !ptr::eq(self.sequence, other.sequence) {
            None
        } else {
            PartialOrd::partial_cmp(&self.pos_chars, &other.pos_chars)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_cursors_are_equal() {
        let c1 = Cursor::new("abc").unwrap();
        let c2 = c1.clone();

        let ordering = PartialOrd::partial_cmp(&c1, &c2);
        assert_eq!(ordering, Some(Ordering::Equal));
    }

    #[test]
    fn further_cursor_is_greater() {
        let c1 = Cursor::new("abc").unwrap();
        let c2 = (c1 + 1).unwrap();

        let ordering = PartialOrd::partial_cmp(&c2, &c1);
        assert_eq!(ordering, Some(Ordering::Greater));

        let ordering = PartialOrd::partial_cmp(&c1, &c2);
        assert_eq!(ordering, Some(Ordering::Less));
    }

    #[test]
    fn different_cursors_dont_compare() {
        let c1 = Cursor::new("abc").unwrap();
        let c2 = Cursor::new("de").unwrap();

        let ordering = PartialOrd::partial_cmp(&c1, &c2);
        assert!(ordering.is_none());
    }
}
