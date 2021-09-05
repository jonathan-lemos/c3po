use crate::parser::parse::parse::Parse;
use super::emptyparser::EmptyParser;
use crate::parser::parser::Parser;
use crate::parser::cursor::cursor::Cursor;

impl<TOutput, FOutputFactory> Parser<TOutput> for EmptyParser<TOutput, FOutputFactory>
where
    TOutput: Send + Sync,
    FOutputFactory: (Fn() -> TOutput) + Clone + Send + Sync,
{
    fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TOutput> {
        Parse::success(cursor, (self.factory)())
    }

    fn kind(&self) -> &str {
        "empty"
    }
}
