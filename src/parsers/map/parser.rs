use super::mapparser::MapParser;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl<TInput, TInputParser, TOutput, FMapper> Parser for MapParser<TInput, TInputParser, TOutput, FMapper>
where
    TInput: Send + Sync,
    TInputParser: Parser<Output = TInput>,
    TOutput: Send + Sync,
    FMapper: (Fn(TInput) -> TOutput) + Clone + Send + Sync,
{
    type Output = TOutput;

    fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TOutput> {
        self.parser.parse(cursor).map(self.mapper.clone())
    }

    fn kind(&self) -> &str {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::string::stringparser::StringParser;
    use super::*;

    #[test]
    fn maps_kind() {
        let cursor = Cursor::new("abcabcabcxabc");

        let sp = StringParser::new("abc");
        let parser = MapParser::new_kind(sp, |_| "abcdef");

        assert_eq!(parser.kind(), "abcdef");
    }

    #[test]
    fn maps_successful_parse() {
        let cursor = Cursor::new("abcabcabcxabc");

        let sp = StringParser::new("abc");
        let parser = MapParser::new_value(sp, |s| s.len());

        let result = parser.parse(cursor).unwrap();
        let value = result.value();

        assert_eq!(value, &3);
    }

    #[test]
    fn keeps_unsuccessful_parse() {
        let sp = StringParser::new("abc");
        let parser = MapParser::new_value(sp, |s| s.len());

        let result = parser.parse(None);

        assert!(result.is_failure());
    }
}
