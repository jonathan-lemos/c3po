use super::manyparser::ManyParser;
use crate::parser::parser::Parser;
use std::marker::PhantomData;

impl<TOutput, TParser> Clone for ManyParser<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<Output = TOutput>,
{
    fn clone(&self) -> Self {
        ManyParser {
            parser: self.parser.clone(),
            kind: self.kind.clone(),
            o: PhantomData,
        }
    }
}
