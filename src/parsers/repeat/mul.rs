use super::repeatparser::RepeatParser;
use crate::parser::parser::Parser;
use std::ops::RangeBounds;

pub trait ParserMul<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<Output = TOutput>,
{
    fn multiply(self, count: usize) -> RepeatParser<TOutput, TParser>;
    fn repeat<T: RangeBounds<usize>>(self, range: T) -> RepeatParser<TOutput, TParser>;
}

impl<TOutput, TParser> ParserMul<TOutput, TParser> for TParser
where
    TOutput: Send + Sync,
    TParser: Parser<Output = TOutput>,
{
    fn multiply(self, count: usize) -> RepeatParser<TOutput, TParser> {
        RepeatParser::count(self, count)
    }

    fn repeat<T: RangeBounds<usize>>(self, range: T) -> RepeatParser<TOutput, TParser> {
        RepeatParser::range(self, range)
    }
}
