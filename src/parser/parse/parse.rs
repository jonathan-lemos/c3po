use super::super::cursor::cursor::Cursor;
use super::failedparse::FailedParse;
use super::successfulparse::SuccessfulParse;

/// Represents the result of a parsing operation.
#[derive(Debug, Clone)]
pub enum Parse<'a, TValue> {
    Success(SuccessfulParse<'a, TValue>),
    Failure(FailedParse<'a>)
}

impl<'a, TValue> Parse<'a, TValue> {
    /// Creates a successful parsing result.
    /// 
    /// # Arguments
    /// * `next`  - A cursor pointing to the next lexeme after the parsed section. `None` if this parse covers the last token in the cursor's source.
    /// * `value` - The parsed value.
    pub fn success(next: Option<Cursor<'a>>, value: TValue) -> Self {
        Parse::Success(SuccessfulParse {
            next,
            value: value.into()
        })
    }

    /// Creates a failed parsing result.
    /// 
    /// # Arguments
    /// * `bad_token` - A cursor pointing to the first unparseable lexeme, or `None` if the parse ran out of lexemes to parse (reached end-of-file).
    /// * `reason`    - The reason why the parse couldn't succeed.
    pub fn failure<S: Into<String>>(bad_token: Option<Cursor<'a>>, reason: S) -> Self {
        Parse::Failure(FailedParse {
            bad_token,
            reason: reason.into()
        })
    }

    /// If successful, return a new Parse as a function of the old Parse, otherwise return the existing failure.
    /// 
    /// For Haskell folk, this binds on the success case.
    /// 
    /// # Arguments
    /// * `if_successful` - If the Parse is successful, run this function.
    pub fn and_then<F: FnOnce(SuccessfulParse<'a, TValue>) -> Parse<'a, TValue>>(self, if_successful: F) -> Self {
        match self {
            Parse::Success(success) => if_successful(success),
            Parse::Failure(failure) => Parse::Failure(failure)
        }
    }

    /// Unwraps a SuccessfulParse if the parse succeeded. Panics with the given message if it failed.
    pub fn expect<S: AsRef<str>>(self, if_not: S) -> SuccessfulParse<'a, TValue> {
        match self {
            Parse::Success(s) => s,
            Parse::Failure(f) => panic!("Expected a successful parse, but it failed due to {}.\nMessage: {}", f.reason(), if_not.as_ref())
        }
    }

    /// Unwraps a FailedParse if the parse failed. Panics with the given message if it succeeded.
    pub fn expect_failure<S: AsRef<str>>(self, if_not: S) -> FailedParse<'a> {
        match self {
            Parse::Failure(f) => f,
            Parse::Success(s) => {
                let pos = s.next().map(|s| s.pos().to_string()).unwrap_or("EOF".to_owned());
                panic!("Expected a failed parse, but it succeeded with before at position {}.\nMessage: {}", pos, if_not.as_ref())
            }
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

    /// If this Parse is a Success, transforms the old value into a new value, otherwise returns the original Failure
    /// 
    /// # Arguments
    /// * `mapper` - A function that transforms the old value into a new one.
    pub fn map<TNewValue, F: FnOnce(TValue) -> TNewValue>(self, mapper: F) -> Parse<'a, TNewValue> {
        match self {
            Parse::Success(success) => Parse::success(success.next, mapper(success.value)),
            Parse::Failure(failure) => Parse::Failure(failure)
        }
    }

    /// If this Parse is a Failure, transforms the old reason into a new reason, otherwise returns the original Success
    /// 
    /// # Arguments
    /// * `mapper` - A function that transforms the old reason into a new one.
    pub fn map_reason<S: Into<String>, F: FnOnce(String) -> S>(self, mapper: F) -> Self {
        match self {
            Parse::Success(s) => Parse::Success(s),
            Parse::Failure(f) => Parse::failure(f.bad_token, mapper(f.reason))
        }
    }

    /// If unsuccessful, return a new Parse as a function of the old Parse, otherwise return the existing success.
    /// 
    /// For Haskell folk, this binds on the failure case.
    /// 
    /// # Arguments
    /// * `if_failure` - If the Parse is unsuccessful, run this function.
    pub fn or_else<F: FnOnce(FailedParse<'a>) -> Parse<'a, TValue>>(self, if_failure: F) -> Self {
        match self {
            Parse::Success(s) => Parse::Success(s),
            Parse::Failure(f) => if_failure(f)
        }
    }

    /// Unwraps a SuccessfulParse if the parse succeeded. Panics if it failed.
    pub fn unwrap(self) -> SuccessfulParse<'a, TValue> {
        self.expect("unwrap() called on a failed parse.")
    }

    /// Unwraps a FailedParse if the parse failed. Panics if it succeeded.
    pub fn unwrap_failure(self) -> FailedParse<'a> {
        self.expect_failure("unwrap_err() called on a successful parse.S")
    }
}