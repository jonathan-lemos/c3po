use super::composeparser::ComposeParser;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl<TLexeme, TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
    Parser<TLexeme, TFinalOutput>
    for ComposeParser<TLexeme, TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
where
    TLexeme: Send + Sync,
    TFirstOutput: Send + Sync,
    TFirst: Parser<TLexeme, TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<TLexeme, TSecondOutput>,
    TFinalOutput: Send + Sync,
    FCombiner: (Fn(TFirstOutput, TSecondOutput) -> TFinalOutput) + Send + Sync + Clone
{
    fn parse<'a>(
        &self,
        cursor: Option<Cursor<'a, TLexeme>>,
    ) -> Parse<'a, TLexeme, TFinalOutput> {
        let first_parse = match self.first.parse(cursor) {
            Parse::Success(success) => success,
            Parse::Failure(failure) => {
                let reason = format!(
                    "Failed to parse {}: {}",
                    self.first.kind(),
                    failure.reason()
                );
                return Parse::failure(cursor, reason);
            }
        };

        self.second
            .parse(*first_parse.next())
            .map_value(|v| (self.combiner)(first_parse.into_value(), v))
            .map_reason(|r| format!("Failed to parse {}: {}", self.second.kind(), r))
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
    fn composes_strings() {
        let chars: Vec<char> = "abcdefghi".chars().collect();
        let cursor = Cursor::new(&chars);

        let sp1 = StringParser::new("abc");
        let sp2 = StringParser::new("def");
        let parser = ComposeParser::new(sp1, sp2);

        let result = parser.parse(cursor).unwrap();
        let (v1, v2) = result.value();

        assert_eq!(v1, "abc");
        assert_eq!(v2, "def");
        assert_eq!(result.next().unwrap().current(), &'g');
    }

    #[test]
    fn fails_if_first_fails() {
        let chars: Vec<char> = "xbcdefghi".chars().collect();
        let cursor = Cursor::new(&chars);

        let sp1 = StringParser::new("abc");
        let sp2 = StringParser::new("def");
        let parser = ComposeParser::new(sp1, sp2);

        assert!(parser.parse(cursor).is_failure())
    }

    #[test]
    fn fails_if_second_fails() {
        let chars: Vec<char> = "abcxefghi".chars().collect();
        let cursor = Cursor::new(&chars);

        let sp1 = StringParser::new("abc");
        let sp2 = StringParser::new("def");
        let parser = ComposeParser::new(sp1, sp2);

        assert!(parser.parse(cursor).is_failure())
    }
}
