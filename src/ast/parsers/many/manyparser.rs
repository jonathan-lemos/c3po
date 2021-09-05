use std::marker::PhantomData;
use crate::parser::parser::Parser;

#[derive(Debug)]
pub struct ManyParser<TOutput: Send + Sync, TParser: Parser<TOutput>> {
    // rust is mega AIDS and says that TOutput is unconstrained, even though it's used for the Parser generic
    // this is supposedly the intended behavior
    pub(super) o: PhantomData<TOutput>,
    pub(super) parser: TParser,
    pub(super) kind: String
}

impl<TOutput: Send + Sync, TParser: Parser<TOutput>> ManyParser<TOutput, TParser> {
    pub fn new(parser: TParser) -> ManyParser<TOutput, TParser> {
        let kind = format!("many {}", parser.kind());
        ManyParser {
            parser,
            o: PhantomData,
            kind
        }
    }
}
