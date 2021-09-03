use super::cursor::Cursor;
use crate::immut_iter::immut_iterable::ImmutableIterable;

impl<TLexeme> ImmutableIterable for Cursor<'_, TLexeme> {
    fn next_immut(&self) -> Option<Self> {
        self + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_iterates_chars() {
        let seq = vec![1, 2, 3];
        let c = Cursor::new(&seq).unwrap();

        let actual: Vec<i32> = c.iter_immut().map(|x| *x.current()).collect();

        assert_eq!(seq, actual);
    }
}
