use super::mapparser::MapParser;
use crate::parser::parser::Parser;

pub trait Map<TInput, TOutput>
where
    TInput: Send + Sync,
    TOutput: Send + Sync,
    Self: Parser<TInput>,
{
    fn map<F>(self, mapper: F) -> MapParser<TInput, Self, TOutput, F>
    where
        F: (Fn(TInput) -> TOutput) + Clone + Send + Sync;
}

impl<TInput, TParser, TOutput> Map<TInput, TOutput> for TParser
where
    TInput: Send + Sync,
    TParser: Parser<TInput>,
    TOutput: Send + Sync,
{
    fn map<F>(self, mapper: F) -> MapParser<TInput, Self, TOutput, F>
    where
        F: (Fn(TInput) -> TOutput) + Clone + Send + Sync,
    {
        MapParser::new(self, mapper)
    }
}
