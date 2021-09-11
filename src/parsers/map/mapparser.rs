use crate::parser::parser::Parser;
use std::marker::PhantomData;

/// Matches a Parser 0 or more times.
#[derive(Debug)]
pub struct MapParser<TInput, TInputParser, TOutput, FMapper>
where
    TInput: Send + Sync,
    TInputParser: Parser<TInput>,
    TOutput: Send + Sync,
    FMapper: (Fn(TInput) -> TOutput) + Clone + Send + Sync,
{
    pub(super) i: PhantomData<TInput>,
    pub(super) o: PhantomData<TOutput>,
    pub(super) parser: TInputParser,
    pub(super) kind: String,
    pub(super) mapper: FMapper,
}

impl<TInput, TInputParser, TOutput, FMapper> MapParser<TInput, TInputParser, TOutput, FMapper>
where
    TInput: Send + Sync,
    TInputParser: Parser<TInput>,
    TOutput: Send + Sync,
    FMapper: (Fn(TInput) -> TOutput) + Clone + Send + Sync,
{
    /// Maps the output of `parser` using a `mapper` function.
    pub fn new(parser: TInputParser, mapper: FMapper) -> Self {
        let kind = parser.kind().to_string();

        MapParser {
            parser,
            kind,
            mapper,
            o: PhantomData,
            i: PhantomData,
        }
    }
}
