use super::manyparser::ManyParser;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl<TOutput, TParser> Parser for ManyParser<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<Output = TOutput>,
{
    type Output = Vec<TOutput>;

    fn parse<'a>(&self, mut cursor: Option<Cursor<'a>>) -> Parse<'a, Vec<TOutput>> {
        let mut results = Vec::new();

        while let Some(c) = cursor {
            let parse = self.parser.parse(Some(c));
            if let Parse::Success(success) = parse {
                cursor = *success.next();
                results.push(success.into_value());
            } else {
                break;
            }
        }

        Parse::success(cursor, results)
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
        let parser = ManyParser::new(sp);

        let result = parser.parse(cursor).unwrap();
        let values = result.value();

        assert_eq!(values.len(), 3);
        assert!(values.iter().all(|x| x == "abc"));
    }

    #[test]
    fn parses_no_abc() {
        let cursor = Cursor::new("xabcabcabcxabc");

        let sp = StringParser::new("abc");
        let parser = ManyParser::new(sp);

        let result = parser.parse(cursor).unwrap();
        let values = result.value();

        assert_eq!(values.len(), 0);
    }

    #[test]
    fn parses_none() {
        let sp = StringParser::new("abc");
        let parser = ManyParser::new(sp);

        let result = parser.parse(None).unwrap();
        let values = result.value();

        assert_eq!(values.len(), 0);
    }
}
