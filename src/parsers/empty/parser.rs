use super::emptyparser::EmptyParser;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl<TOutput, FOutputFactory> Parser for EmptyParser<TOutput, FOutputFactory>
where
    TOutput: Send + Sync,
    FOutputFactory: (Fn() -> TOutput) + Clone + Send + Sync,
{
    type Output = TOutput;

    fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TOutput> {
        Parse::success(cursor, (self.factory)())
    }

    fn kind(&self) -> &str {
        "empty"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nothing_on_input() {
        let cursor = Cursor::new("abc");

        let parser = EmptyParser::new();
        let result = parser.parse(cursor).unwrap();

        assert_eq!(result.next().unwrap().current(), 'a')
    }

    #[test]
    fn succeeds_on_eof() {
        let cursor = Cursor::new("");

        let parser = EmptyParser::new();
        let result = parser.parse(cursor).unwrap();

        assert_eq!(result.next(), &None)
    }

    #[test]
    fn returns_factory_value() {
        let cursor = Cursor::new("abc");
        let value = 4;

        let parser = EmptyParser::using_factory(|| value);
        let result = parser.parse(cursor).unwrap();

        assert_eq!(result.value(), &4);
        assert_eq!(result.next().unwrap().current(), 'a');
    }
}
