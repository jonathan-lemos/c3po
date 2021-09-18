use super::super::cursor::cursor::Cursor;

/// Represents a successful parse result. See the `Parse` enum for details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessfulParse<'a, TValue> {
    pub next: Option<Cursor<'a>>,
    pub value: TValue,
}

impl<'a, TValue> SuccessfulParse<'a, TValue> {
    /// Creates a SuccessfulParse
    /// 
    /// # Arguments
    /// * `next`  - A cursor pointing to the next lexeme after the parsed section. `None` if this parse covers the last token in the cursor's source.
    /// * `value` - The parsed value.
    pub fn new<V: Into<TValue>>(next: Option<Cursor<'a>>, value: V) -> Self {
        SuccessfulParse {
            next,
            value: value.into()
        }
    }
}
