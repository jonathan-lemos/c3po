use super::super::cursor::cursor::Cursor;

/// Represents a successful parse result. See the `Parse` enum for details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessfulParse<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TValue> {
    pub(super) end: TCursor,
    pub(super) value: TValue
}

impl<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TValue> SuccessfulParse<TLexeme, TCursor, TValue> {
    /// Creates a SuccessfulParse
    /// 
    /// # Arguments
    /// * `end`   - The last parsed lexeme.
    /// * `value` - The parsed value.
    pub fn new<V: Into<TValue>>(end: TCursor, value: V) -> Self {
        SuccessfulParse {
            end: end,
            value: value.into()
        }
    }

    /// A Cursor pointing to the last parsed lexeme.
    pub fn end(&self) -> &TCursor {
        &self.end
    }

    /// The parsed value.
    pub fn value(&self) -> &TValue {
        &self.value
    }
}
