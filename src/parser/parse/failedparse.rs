use super::super::cursor::cursor::Cursor;

/// Represents a failed parse result. See the `Parse` enum for details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedParse<'a, TLexeme> {
    pub(super) bad_token: Option<Cursor<'a, TLexeme>>,
    pub(super) reason: String,
}

impl<'a, TLexeme> FailedParse<'a, TLexeme> {
    /// Creates a FailedParse
    /// 
    /// # Arguments
    /// * `bad_token` - A cursor pointing to the first unparseable lexeme, or `None` if the parse ran out of lexemes to parse (reached end-of-file).
    /// * `reason`    - A reason why the lexemes couldn't be parsed.
    pub fn new<S: Into<String>>(bad_token: Option<Cursor<'a, TLexeme>>, reason: S) -> Self {
        FailedParse {
            bad_token,
            reason: reason.into()
        }
    }

    /// A cursor pointing to the first unparseable lexeme, or `None` if the parse ran out of lexemes to parse (reached end-of-file).
    pub fn bad_token(&self) -> &Option<Cursor<'a, TLexeme>> {
        &self.bad_token
    }

    /// The reason why the lexemes couldn't be parsed.
    pub fn reason(&self) -> &str {
        &self.reason
    }
}
