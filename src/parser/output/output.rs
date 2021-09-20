use super::super::cursor::cursor::Cursor;
use super::super::parse::parse::Parse;

/// The output type of `Parser::parse`.
pub struct ParserOutput<'a, TOutput> {
    beginning: Cursor<'a>,
    kind: String,
    parse: Parse<'a, TOutput>,
}

impl<'a, TOutput> ParserOutput<'a, TOutput> {
    /// Creates a new ParserOutput
    ///
    /// # Arguments
    /// * `parse`     - A parse generated by a parsing function. See `Parser::func` for details.
    /// * `beginning` - A Cursor pointing to the first token of the parse.
    /// * `kind`      - What kind of value this parse has (on success).
    pub fn new<S: Into<String>>(parse: Parse<'a, TOutput>, beginning: Cursor<'a>, kind: S) -> Self {
        ParserOutput {
            beginning,
            kind: kind.into(),
            parse,
        }
    }

    /// A Cursor pointing to the first token of the parse.
    pub fn beginning(&self) -> &Cursor<'a> {
        &self.beginning
    }

    /// What kind of value this parse should have.
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// The parse itself.
    pub fn parse(&self) -> &Parse<'a, TOutput> {
        &self.parse
    }
}
