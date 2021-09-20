use super::cursor::Cursor;

impl<'a> Clone for Cursor<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for Cursor<'a> {}
