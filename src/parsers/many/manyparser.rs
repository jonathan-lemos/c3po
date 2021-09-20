use crate::parser::parser::Parser;
use std::marker::PhantomData;

/// Matches a Parser 0 or more times.
#[derive(Debug)]
pub struct ManyParser<TOutput: Send + Sync, TParser: Parser<Output = TOutput>> {
    // rust is mega AIDS and says that TOutput is unconstrained, even though it's used for the Parser generic
    // this is supposedly the intended behavior
    pub(super) o: PhantomData<TOutput>,
    pub(super) parser: TParser,
    pub(super) kind: String,
}

impl<TOutput: Send + Sync, TParser: Parser<Output = TOutput>> ManyParser<TOutput, TParser> {
    /// Creates a new ManyParser that matches the given parser 0 or more times.
    pub fn new(parser: TParser) -> ManyParser<TOutput, TParser> {
        let kind = format!("many({})", parser.kind());
        ManyParser {
            parser,
            o: PhantomData,
            kind,
        }
    }
}
