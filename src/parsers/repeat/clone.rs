use super::repeatparser::RepeatParser;
use crate::parser::parser::Parser;
use std::marker::PhantomData;

impl<TOutput, TParser> Clone for RepeatParser<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<Output = TOutput>,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            kind: self.kind.clone(),
            bounds: self.bounds.clone(),
            o: PhantomData,
        }
    }
}
