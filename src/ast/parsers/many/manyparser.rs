use std::marker::PhantomData;
use crate::parser::parser::Parser;

#[derive(Debug)]
pub struct ManyParser<TLexeme: Send + Sync, TOutput: Send + Sync, TParser: Parser<TLexeme, TOutput>> {
    // rust is mega AIDS and says that TLexeme is unconstrained, even though it's used for the Parser generic
    // this is supposedly the intended behavior
    pub(super) l: PhantomData<TLexeme>,
    pub(super) o: PhantomData<TOutput>,
    pub(super) parser: TParser,
    pub(super) kind: String
}

impl<TLexeme: Send + Sync, TOutput: Send + Sync, TParser: Parser<TLexeme, TOutput>> ManyParser<TLexeme, TOutput, TParser> {
    pub fn new(parser: TParser) -> ManyParser<TLexeme, TOutput, TParser> {
        let kind = format!("many {}", parser.kind());
        ManyParser {
            parser,
            l: PhantomData,
            o: PhantomData,
            kind
        }
    }
}
