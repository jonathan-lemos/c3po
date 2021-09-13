use super::someparser::SomeParser;
use crate::parser::parser::Parser;

impl<TOutput, TParser> Clone for SomeParser<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<Output = TOutput>,
{
    fn clone(&self) -> Self {
        SomeParser {
            parser: self.parser.clone(),
            kind: self.kind.clone(),
            base_kind: self.base_kind.clone()
        }
    }
}
