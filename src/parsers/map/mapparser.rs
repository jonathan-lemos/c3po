use crate::parser::parser::Parser;
use std::marker::PhantomData;

/// Matches a Parser 0 or more times.
#[derive(Debug)]
pub struct MapParser<TInput, TInputParser, TOutput, FMapper>
where
    TInput: Send + Sync,
    TInputParser: Parser<Output = TInput>,
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
    TInputParser: Parser<Output = TInput>,
    TOutput: Send + Sync,
    FMapper: (Fn(TInput) -> TOutput) + Clone + Send + Sync,
{
    /// Maps the output of `parser` using a `value_mapper` function. Also maps the kind of `parser` using a `kind_mapper` function.
    ///
    /// # Arguments
    /// * `parser`       - The parser to transform.
    /// * `kind_mapper`  - A function that transforms the old `kind()` into a new `kind()`.
    /// * `value_mapper` - A function that transforms the old output into a new output.
    pub fn new<TKind: Into<String>, FKindMapper: FnOnce(&str) -> TKind>(
        parser: TInputParser,
        kind_mapper: FKindMapper,
        value_mapper: FMapper,
    ) -> Self {
        let kind = parser.kind();
        let new_kind = kind_mapper(kind).into();

        MapParser {
            parser,
            kind: new_kind,
            mapper: value_mapper,
            o: PhantomData,
            i: PhantomData,
        }
    }

    /// Maps the output of `parser` using a `value_mapper` function. The `kind()` of the new parser will be equal to the old one.
    ///
    /// # Arguments
    /// * `parser`       - The parser to transform.
    /// * `value_mapper` - A function that transforms the old output into a new output.
    pub fn new_value(parser: TInputParser, value_mapper: FMapper) -> Self {
        Self::new(parser, |s| s.to_string(), value_mapper)
    }
}

impl<TInput, TInputParser> MapParser<TInput, TInputParser, TInput, fn(TInput) -> TInput>
where
    TInput: Send + Sync,
    TInputParser: Parser<Output = TInput>,
{
    /// Maps the `kind()` of `parser` with a `kind_mapper` function. The output is the same as the old parser.
    ///
    /// # Arguments
    /// * `parser`       - The parser to transform.
    /// * `kind_mapper`  - A function that transforms the old `kind()` into a new `kind()`.
    pub fn new_kind<TKind: Into<String>, FKindMapper: FnOnce(&str) -> TKind>(
        parser: TInputParser,
        kind_mapper: FKindMapper,
    ) -> Self {
        Self::new(parser, kind_mapper, |v| v)
    }
}
