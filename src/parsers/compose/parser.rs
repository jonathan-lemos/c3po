use super::composeparser::ComposeParser;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner> Parser
    for ComposeParser<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
where
    TFirstOutput: Send + Sync,
    TFirst: Parser<Output = TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<Output = TSecondOutput>,
    TFinalOutput: Send + Sync,
    FCombiner: (Fn(TFirstOutput, TSecondOutput) -> TFinalOutput) + Send + Sync + Clone,
{
    type Output = TFinalOutput;

    fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TFinalOutput> {
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
            .map(|v| (self.combiner)(first_parse.into_value(), v))
            .map_reason(|r| format!("Failed to parse {}: {}", self.second.kind(), r))
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
    fn composes_strings() {
        let cursor = Cursor::new("abcdefghi");

        let sp1 = StringParser::new("abc");
        let sp2 = StringParser::new("def");
        let parser = ComposeParser::new(sp1, sp2);

        let result = parser.parse(cursor).unwrap();
        let (v1, v2) = result.value();

        assert_eq!(v1, "abc");
        assert_eq!(v2, "def");
        assert_eq!(result.next().unwrap().current(), 'g');
    }

    #[test]
    fn fails_if_first_fails() {
        let cursor = Cursor::new("xbcdefghi");

        let sp1 = StringParser::new("abc");
        let sp2 = StringParser::new("def");
        let parser = ComposeParser::new(sp1, sp2);

        assert!(parser.parse(cursor).is_failure())
    }

    #[test]
    fn fails_if_second_fails() {
        let cursor = Cursor::new("abcxefghi");

        let sp1 = StringParser::new("abc");
        let sp2 = StringParser::new("def");
        let parser = ComposeParser::new(sp1, sp2);

        assert!(parser.parse(cursor).is_failure())
    }
}
