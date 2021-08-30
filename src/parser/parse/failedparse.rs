use super::super::cursor::cursor::Cursor;

/// Represents a failed parse result. See the `Parse` enum for details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedParse<'a, TLexeme> {
    pub(super) bad_token: Cursor<'a, TLexeme>,
    pub(super) reason: String,
}

impl<'a, TLexeme> FailedParse<'a, TLexeme> {
    /// Creates a FailedParse
    /// 
    /// # Arguments
    /// * `bad_token` - A Cursor pointing to the first unparseable token.
    /// * `reason`    - A reason why the lexemes couldn't be parsed.
    pub fn new<S: Into<String>>(bad_token: Cursor<'a, TLexeme>, reason: S) -> Self {
        FailedParse {
            bad_token,
            reason: reason.into()
        }
    }

    /// A Cursor pointing to the first unparseable lexeme.
    pub fn bad_token(&self) -> &Cursor<'a, TLexeme> {
        &self.bad_token
    }

    /// The reason why the lexemes couldn't be parsed.
    pub fn reason(&self) -> &str {
        &self.reason
    }
}
