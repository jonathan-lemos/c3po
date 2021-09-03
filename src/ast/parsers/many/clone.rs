use std::marker::PhantomData;
use super::manyparser::ManyParser;
use crate::parser::parser::Parser;

impl<TLexeme, TOutput, TParser> Clone for ManyParser<TLexeme, TOutput, TParser>
where
    TLexeme: Send + Sync,
    TOutput: Send + Sync,
    TParser: Parser<TLexeme, TOutput>,
{
    fn clone(&self) -> Self {
        ManyParser {
            parser: self.parser.clone(),
            kind: self.kind.clone(),
            l: PhantomData,
            o: PhantomData
        }
    }
}
