use super::cursor::cursor::Cursor;
use super::parse::parse::Parse;

pub struct Parser<'f, TLexeme: 'f, TCursor: Cursor<Lexeme = TLexeme> + 'f, TOutput: 'f> {
    pub(super) func: Box<dyn Fn(&TCursor) -> Parse<TLexeme, TCursor, TOutput> + 'f>,
    pub(super) kind: String
}

impl<'f, TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TOutput> Parser<'f, TLexeme, TCursor, TOutput> {
    /// Creates a new parser
    /// 
    /// # Arguments
    /// * func - A function taking a `&Cursor` input, and returning a `Result` with a value on success, and an error message on failure.
    /// * kind - An `AsRef<str>` describing the kind of values this parser accepts e.g. comma, identifier, keyword, number, string, etc.
    fn new<F: Fn(&TCursor) -> Parse<TLexeme, TCursor, TOutput> + 'f, S: AsRef<str>>(func: F, kind: S) -> Self {
        Parser {
            func: Box::new(func),
            kind: kind.as_ref().to_string()
        }
    }

    /// Parses a value starting at the given cursor.
    /// 
    /// To get the next token beyond a successful parse, get the `next_immut()` of the returned `SuccessfulParse::end()`
    fn parse(&self, cursor: &TCursor) -> Parse<TLexeme, TCursor, TOutput> {
        (*self.func)(cursor)
    }
}
