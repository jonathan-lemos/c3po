use super::cursor::Cursor;
use crate::immut_iter::immut_iterable::ImmutableIterable;

impl ImmutableIterable for Cursor<'_> {
    fn next_immut(&self) -> Option<Self> {
        let length_bytes = self.current().len_utf8();

        let ret = Cursor {
            sequence: self.sequence,
            pos_bytes: self.pos_bytes + length_bytes,
            pos_chars: self.pos_chars + 1,
        };

        if ret.pos_bytes >= ret.sequence.len() {
            None
        } else {
            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_iterates_chars() {
        let c = Cursor::new("abc").unwrap();

        let actual: String = c.iter_immut().map(|x| x.current()).collect();

        assert_eq!("abc", &actual);
    }
}
