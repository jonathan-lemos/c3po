use super::super::cursor::cursor::Cursor;

/// Represents a failed parse result. See the `Parse` enum for details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedParse<'a> {
    pub bad_token: Option<Cursor<'a>>,
    pub reason: String,
}
