use super::super::cursor::cursor::Cursor;

/// Represents a successful parse result. See the `Parse` enum for details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessfulParse<'a, TLexeme, TValue> {
    pub(super) end: Cursor<'a, TLexeme>,
    pub(super) value: TValue
}

impl<'a, TLexeme, TValue> SuccessfulParse<'a, TLexeme, TValue> {
    /// Creates a SuccessfulParse
    /// 
    /// # Arguments
    /// * `end`   - The last parsed lexeme.
    /// * `value` - The parsed value.
    pub fn new<V: Into<TValue>>(end: Cursor<'a, TLexeme>, value: V) -> Self {
        SuccessfulParse {
            end: end,
            value: value.into()
        }
    }

    /// A Cursor pointing to the last parsed lexeme.
    pub fn end(&self) -> &Cursor<'a, TLexeme> {
        &self.end
    }

    /// The parsed value.
    pub fn value(&self) -> &TValue {
        &self.value
    }
}
