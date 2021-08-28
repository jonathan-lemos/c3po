use super::super::cursor::cursor::Cursor;

/// Represents a failed parse result. See the `Parse` enum for details.
#[derive(Debug, Clone)]
pub struct FailedParse<TLexeme, TCursor: Cursor<Lexeme = TLexeme>> {
    bad_token: TCursor,
    beginning: TCursor,
    kind: String,
    reason: String,
}

impl<TLexeme, TCursor: Cursor<Lexeme = TLexeme>> FailedParse<TLexeme, TCursor> {
    /// Returns the first unparseable lexeme.
    pub fn bad_token(&self) -> &TCursor {
        &self.bad_token
    }

    /// Returns the first lexeme of the result.
    pub fn beginning(&self) -> &TCursor {
        &self.beginning
    }

    /// Returns the kind of value this would have had if it was successful.
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns the reason why the parse could not occur.
    pub fn reason(&self) -> &str {
        &self.reason
    }
}
