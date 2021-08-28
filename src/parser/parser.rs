use super::cursor::cursor::Cursor;
use super::parse::parse::Parse;

#[derive(Clone)]
pub struct Parser<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TOutput> {
    pub(super) func: fn(&TCursor) -> Parse<TLexeme, TCursor, TOutput>,
    pub(super) kind: String
}

impl<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TOutput> Parser<TLexeme, TCursor, TOutput> {
    /// Creates a new parser
    /// 
    /// # Arguments
    /// * func - A function taking a `&Cursor` input, and returning a `Result` with a value on success, and an error message on failure.
    /// * kind - An `AsRef<str>` describing the kind of values this parser accepts e.g. comma, identifier, keyword, number, string, etc.
    fn new<S: AsRef<str>>(func: fn(&TCursor) -> Parse<TLexeme, TCursor, TOutput>, kind: S) -> Self {
        Parser {
            func,
            kind: kind.as_ref().to_string()
        }
    }

    /// Parses a value starting at the given cursor.
    /// 
    /// To get the next token beyond a successful parse, get the `next_immut()` of the returned `SuccessfulParse::end()`
    fn parse(&self, cursor: &TCursor) -> Parse<TLexeme, TCursor, TOutput> {
        (self.func)(cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::cursor::reference::refcursor::RefCursor;

    #[test]
    fn parser_implements_send_sync() {
        assert_impl_all!(Parser<char, RefCursor<char>, String>: Send, Sync);
    }
}
