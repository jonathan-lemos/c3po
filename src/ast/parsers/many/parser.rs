use super::manyparser::ManyParser;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;
use crate::parser::cursor::cursor::Cursor;

impl<TLexeme, TOutput, TParser> Parser<TLexeme, Vec<TOutput>> for ManyParser<TLexeme, TOutput, TParser>
where
    TLexeme: Send + Sync,
    TOutput: Send + Sync,
    TParser: Parser<TLexeme, TOutput>
 {
    fn parse<'a>(&self, mut cursor: Option<Cursor<'a, TLexeme>>) -> Parse<'a, TLexeme, Vec<TOutput>> {
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
    use super::*;
    use super::super::super::string::stringparser::StringParser;

    #[test]
    fn parses_many_abc() {
        let chars: Vec<char> = "abcabcabcxabc".chars().collect();
        let cursor = Cursor::new(&chars);

        let sp = StringParser::new("abc");
        let parser = ManyParser::new(sp);

        let result = parser.parse(cursor).unwrap();
        let values = result.value();

        assert_eq!(values.len(), 3);
        assert!(values.iter().all(|x| x == "abc"));
    }

    #[test]
    fn parses_no_abc() {
        let chars: Vec<char> = "xabcabcabcxabc".chars().collect();
        let cursor = Cursor::new(&chars);

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