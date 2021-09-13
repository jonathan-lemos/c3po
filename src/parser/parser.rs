use super::cursor::cursor::Cursor;
use super::parse::parse::Parse;

pub trait Parser: Clone + Send + Sync {
    type Output;

    fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, Self::Output>;
    fn kind(&self) -> &str;
}
