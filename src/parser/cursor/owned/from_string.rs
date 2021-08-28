use super::ownedcursor::OwnedCursor;

impl<S: AsRef<str>> From<S> for OwnedCursor<char> {
    fn from(string: S) -> Self {
        OwnedCursor::new(string.as_ref().to_string().chars())
    }
}
