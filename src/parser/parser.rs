use super::cursor::cursor::Cursor;
use super::parse::parse::Parse;
use super::output::output::ParserOutput;

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
    fn new<S: Into<String>>(func: fn(&TCursor) -> Parse<TLexeme, TCursor, TOutput>, kind: S) -> Self {
        Parser {
            func,
            kind: kind.into()
        }
    }

    /// Parses a value starting at the given cursor.
    /// 
    /// To get the next token beyond a successful parse, get the `next_immut()` of the returned `SuccessfulParse::end()`
    fn parse(&self, cursor: &TCursor) -> ParserOutput<TLexeme, TCursor, TOutput> {
        let parse = (self.func)(cursor);
        ParserOutput::new(parse, cursor.clone(), &self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::cursor::owned::ownedcursor::OwnedCursor;

    #[test]
    fn parser_implements_send_sync() {
        assert_impl_all!(Parser<char, OwnedCursor<char>, String>: Send, Sync);
    }

    #[test]
    fn parser_returns_successful_parse_on_success() {
        let cursor: OwnedCursor<char> = From::from("ab");

        let parser = Parser::new(|c: &OwnedCursor<char>| {
            Parse::success(c.next_immut().unwrap(), "test value")
        }, "kind");

        let output = parser.parse(&cursor);

        assert!(output.parse().is_success())
    }
}
