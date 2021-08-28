use super::super::cursor::cursor::Cursor;
use super::failedparse::FailedParse;
use super::successfulparse::SuccessfulParse;

/// Represents the result of a parsing operation.
#[derive(Debug, Clone)]
pub enum Parse<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TValue> {
    Success(SuccessfulParse<TLexeme, TCursor, TValue>),
    Failure(FailedParse<TLexeme, TCursor>)
}

impl<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TValue> Parse<TLexeme, TCursor, TValue> {
    /// Creates a successful parsing result.
    /// 
    /// # Arguments
    /// * beginning - A cursor pointing to the first lexeme of the parse.
    /// * end       - A cursor pointing to the last lexeme of the parse.
    /// * kind      - The kind of value this is e.g. comma, identifier, keyword, number, string, etc..
    /// * value     - The parsed value.
    fn success<S: AsRef<str>, V: Into<TValue>>(beginning: TCursor, end: TCursor, kind: S, value: V) -> Self {
        Parse::Success(SuccessfulParse {
            beginning,
            end,
            kind: kind.as_ref().to_string(),
            value: value.into()
        })
    }

    /// Creates a failed parsing result.
    /// 
    /// # Arguments
    /// * beginning - A cursor pointing to the first lexeme of the parse.
    /// * bad_token - A cursor pointing to the first unparseable lexeme.
    /// * reason    - The reason why the parse couldn't succeed.
    fn failure<A: AsRef<str>, B: AsRef<str>>(beginning: TCursor, bad_token: TCursor, kind: A, reason: B) -> Self {
        Parse::Failure(FailedParse {
            beginning,
            bad_token,
            kind: kind.as_ref().to_string(),
            reason: reason.as_ref().to_string()
        })
    }
}