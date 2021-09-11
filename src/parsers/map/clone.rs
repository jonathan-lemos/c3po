use std::marker::PhantomData;
use super::mapparser::MapParser;
use crate::parser::parser::Parser;

impl<TInput, TInputParser, TOutput, FMapper> Clone for MapParser<TInput, TInputParser, TOutput, FMapper>
where
    TInput: Send + Sync,
    TInputParser: Parser<TInput>,
    TOutput: Send + Sync,
    FMapper: (Fn(TInput) -> TOutput) + Clone + Send + Sync,
{
    fn clone(&self) -> Self {
        MapParser {
            parser: self.parser.clone(),
            kind: self.kind.clone(),
            mapper: self.mapper.clone(),
            o: PhantomData,
            i: PhantomData
        }
    }
}
