use super::someparser::SomeParser;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl<TOutput, TParser> Parser<Vec<TOutput>> for SomeParser<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<TOutput>,
{
    fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, Vec<TOutput>> {
        self.parser.parse(cursor).map_reason(|r| {
            format!(
                "Expected at least one {}, but didn't read it: {}",
                self.base_kind, r
            )
        })
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
    fn parses_many_abc() {
        let cursor = Cursor::new("abcabcabcxabc");

        let sp = StringParser::new("abc");
        let parser = SomeParser::new(sp);

        let result = parser.parse(cursor).unwrap();
        let values = result.value();

        assert_eq!(values.len(), 3);
        assert!(values.iter().all(|x| x == "abc"));
    }

    #[test]
    fn fails_on_no_abc() {
        let cursor = Cursor::new("xabcabcabcxabc");

        let sp = StringParser::new("abc");
        let parser = SomeParser::new(sp);

        assert!(parser.parse(cursor).is_failure());
    }

    #[test]
    fn fails_on_none() {
        let sp = StringParser::new("abc");
        let parser = SomeParser::new(sp);

        assert!(parser.parse(None).is_failure());
    }
}
