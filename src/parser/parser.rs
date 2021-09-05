use super::cursor::cursor::Cursor;
use super::parse::parse::Parse;

pub trait Parser<TOutput>: Clone + Send + Sync {
    fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TOutput>;
    fn kind(&self) -> &str;
}
