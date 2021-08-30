use super::cursor::cursor::Cursor;
use super::parse::parse::Parse;
use super::output::output::ParserOutput;

/// Parses a value of `TOutput` starting from a given `Cursor<TLexeme>`.
pub struct Parser<TLexeme, TOutput> {
    pub(super) func: for<'a> fn(&'a Cursor<TLexeme>) -> Parse<'a, TLexeme, TOutput>,
    pub(super) kind: String
}

impl<TLexeme, TOutput> Parser<TLexeme, TOutput> {
    /// Creates a new parser
    /// 
    /// # Arguments
    /// * func - A function taking a `&Cursor` input, and returning a `Result` with a value on success, and an error message on failure.
    /// * kind - An `AsRef<str>` describing the kind of values this parser accepts e.g. comma, identifier, keyword, number, string, etc.
    fn new<S: Into<String>>(func: for<'a> fn(&'a Cursor<TLexeme>) -> Parse<'a, TLexeme, TOutput>, kind: S) -> Self {
        Parser {
            func,
            kind: kind.into()
        }
    }

    /// Parses a value starting at the given cursor.
    /// 
    /// To get the next token beyond a successful parse, get the `next_immut()` of the returned `SuccessfulParse::end()`
    fn parse<'a>(&self, cursor: &'a Cursor<TLexeme>) -> ParserOutput<'a, TLexeme, TOutput> {
        let parse = (self.func)(cursor);
        ParserOutput::new(parse, *cursor, &self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::cursor::cursor::Cursor;

    #[test]
    fn parser_implements_send_sync() {
        assert_impl_all!(Parser<char, String>: Send, Sync);
    }

    #[test]
    fn parser_returns_successful_parse_on_success() {
        let chars: Vec<char> = "ab".chars().collect();
        let cursor = Cursor::new(&chars).unwrap();

        let parser = Parser::new(|c| {
            Parse::success(c.next_immut().unwrap(), "test value")
        }, "kind");

        let output = parser.parse(&cursor);

        assert!(output.parse().is_success())
    }
}
