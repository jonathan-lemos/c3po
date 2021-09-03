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
