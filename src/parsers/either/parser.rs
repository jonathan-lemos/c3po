use super::eitherparser::EitherParser;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl<TLeftOutput, TLeft, TRightOutput, TRight, TFinalOutput, FLeftMapper, FRightMapper>
    Parser<TFinalOutput>
    for EitherParser<
        TLeftOutput,
        TLeft,
        TRightOutput,
        TRight,
        TFinalOutput,
        FLeftMapper,
        FRightMapper,
    >
where
    TLeftOutput: Send + Sync,
    TLeft: Parser<TLeftOutput>,
    TRightOutput: Send + Sync,
    TRight: Parser<TRightOutput>,
    TFinalOutput: Send + Sync,
    FLeftMapper: (Fn(TLeftOutput) -> TFinalOutput) + Send + Sync + Clone,
    FRightMapper: (Fn(TRightOutput) -> TFinalOutput) + Send + Sync + Clone,
{
    fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TFinalOutput> {
        self.left
            .parse(cursor)
            .map(self.if_left.clone())
            .or_else(|e| {
                self.right
                    .parse(cursor)
                    .map(self.if_right.clone())
                    .map_reason(|r| {
                        format!(
                            "Couldn't parse {} because {}. Couldn't parse {} because {}",
                            self.left.kind(),
                            e.reason(),
                            self.right.kind(),
                            r
                        )
                    })
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
    fn reads_first() {
        let cursor = Cursor::new("abcdefghi");

        let sp1 = StringParser::new("abc");
        let sp2 = StringParser::new("def");
        let parser = EitherParser::new(sp1, sp2);

        let result = parser.parse(cursor).unwrap();
        let value = result.value().as_ref().unwrap();

        assert_eq!(value, "abc");
        assert_eq!(result.next().unwrap().current(), 'd');
    }

    #[test]
    fn reads_second() {
        let cursor = Cursor::new("defghi");

        let sp1 = StringParser::new("abc");
        let sp2 = StringParser::new("def");
        let parser = EitherParser::new(sp1, sp2);

        let result = parser.parse(cursor).unwrap();
        // not actually an error, just the second part of the Result<> i'm using
        let value = result.value().as_ref().unwrap_err();

        assert_eq!(value, "def");
        assert_eq!(result.next().unwrap().current(), 'g');
    }

    #[test]
    fn fails_if_both_fail() {
        let cursor = Cursor::new("xbcdefghi");

        let sp1 = StringParser::new("abc");
        let sp2 = StringParser::new("def");
        let parser = EitherParser::new(sp1, sp2);

        assert!(parser.parse(cursor).is_failure())
    }
}
