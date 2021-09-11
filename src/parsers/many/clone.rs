use std::marker::PhantomData;
use super::manyparser::ManyParser;
use crate::parser::parser::Parser;

impl<TOutput, TParser> Clone for ManyParser<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<TOutput>,
{
    fn clone(&self) -> Self {
        ManyParser {
            parser: self.parser.clone(),
            kind: self.kind.clone(),
            o: PhantomData
        }
    }
}
