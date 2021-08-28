use super::super::cursor::cursor::Cursor;

/// Represents a successful parse result. See the `Parse` enum for details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessfulParse<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TValue> {
    pub(super) beginning: TCursor,
    pub(super) end: TCursor,
    pub(super) kind: String,
    pub(super) value: TValue
}

impl<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TValue> SuccessfulParse<TLexeme, TCursor, TValue> {
    /// Returns the first lexeme of the result.
    pub fn beginning(&self) -> &TCursor {
        &self.beginning
    }

    /// Returns the last lexeme of the result.
    pub fn end(&self) -> &TCursor {
        &self.end
    }

    /// Returns the kind of value this result contains.
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns the value of the result.
    pub fn value(&self) -> &TValue {
        &self.value
    }
}
