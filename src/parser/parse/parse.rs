use super::super::cursor::cursor::Cursor;
use super::failedparse::FailedParse;
use super::successfulparse::SuccessfulParse;

/// Represents the result of a parsing operation.
#[derive(Debug, Clone)]
pub enum Parse<'a, TLexeme, TValue> {
    Success(SuccessfulParse<'a, TLexeme, TValue>),
    Failure(FailedParse<'a, TLexeme>)
}

impl<'a, TLexeme, TValue> Parse<'a, TLexeme, TValue> {
    /// Creates a successful parsing result.
    /// 
    /// # Arguments
    /// * end       - A cursor pointing to the last lexeme of the parse.
    /// * value     - The parsed value.
    pub fn success(end: Cursor<'a, TLexeme>, value: TValue) -> Self {
        Parse::Success(SuccessfulParse {
            end,
            value: value.into()
        })
    }

    /// Creates a failed parsing result.
    /// 
    /// # Arguments
    /// * bad_token - A cursor pointing to the first unparseable lexeme.
    /// * reason    - The reason why the parse couldn't succeed.
    pub fn failure<S: Into<String>>(bad_token: Cursor<'a, TLexeme>, reason: S) -> Self {
        Parse::Failure(FailedParse {
            bad_token,
            reason: reason.into()
        })
    }

    /// Unwraps a SuccessfulParse if the parse succeeded. Panics with the given message if it failed.
    pub fn expect<S: AsRef<str>>(self, if_not: S) -> SuccessfulParse<'a, TLexeme, TValue> {
        match self {
            Parse::Success(s) => s,
            Parse::Failure(f) => panic!("Expected a successful parse, but it failed due to {}.\nMessage: {}", f.reason(), if_not.as_ref())
        }
    }

    /// Unwraps a FailedParse if the parse failed. Panics with the given message if it succeeded.
    pub fn expect_failure<S: AsRef<str>>(self, if_not: S) -> FailedParse<'a, TLexeme> {
        match self {
            Parse::Failure(f) => f,
            Parse::Success(s) => panic!("Expected a failed parse, but it succeeded with ending at position {}.\nMessage: {}", s.end().pos(), if_not.as_ref())
        }
    }

    /// `true` if the Parse failed, `false` if not.
    pub fn is_failure(&self) -> bool {
        match self {
            Parse::Failure(_) => true,
            Parse::Success(_) => false
        }
    }

    /// `true` if the Parse succeeded, `false` if not.
    pub fn is_success(&self) -> bool {
        match self {
            Parse::Success(_) => true,
            Parse::Failure(_) => false
        }
    }

    /// Unwraps a SuccessfulParse if the parse succeeded. Panics if it failed.
    pub fn unwrap(self) -> SuccessfulParse<'a, TLexeme, TValue> {
        self.expect("unwrap() called on a failed parse.")
    }

    /// Unwraps a FailedParse if the parse failed. Panics if it succeeded.
    pub fn unwrap_failure(self) -> FailedParse<'a, TLexeme> {
        self.expect_failure("unwrap_err() called on a successful parse.S")
    }
}