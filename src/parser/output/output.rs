use super::super::cursor::cursor::Cursor;
use super::super::parse::parse::Parse;

/// The output type of `Parser::parse`.
pub struct ParserOutput<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TOutput> {
    beginning: TCursor,
    kind: String,
    parse: Parse<TLexeme, TCursor, TOutput>,
}

impl<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TOutput> ParserOutput<TLexeme, TCursor, TOutput> {
    /// Creates a new ParserOutput
    /// 
    /// # Arguments
    /// * `parse`     - A parse generated by a parsing function. See `Parser::func` for details.
    /// * `beginning` - A Cursor pointing to the first token of the parse.
    /// * `kind`      - What kind of value this parse has (on success).
    pub fn new<C: Into<TCursor>, S: Into<String>>(parse: Parse<TLexeme, TCursor, TOutput>, beginning: C, kind: S) -> Self {
        ParserOutput {
            beginning: beginning.into(),
            kind: kind.into(),
            parse
        }
    }

    /// A Cursor pointing to the first token of the parse.
    pub fn beginning(&self) -> &TCursor {
        &self.beginning
    }

    /// What kind of value this parse should have.
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// The parse itself.
    pub fn parse(&self) -> &Parse<TLexeme, TCursor, TOutput> {
        &self.parse
    }
}