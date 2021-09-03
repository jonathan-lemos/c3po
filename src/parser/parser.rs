use super::cursor::cursor::Cursor;
use super::parse::parse::Parse;

pub trait Parser<TLexeme, TOutput>: Clone + Send + Sync {
    fn parse<'a>(&self, cursor: Option<Cursor<'a, TLexeme>>) -> Parse<'a, TLexeme, TOutput>;
    fn kind(&self) -> &str;
}
