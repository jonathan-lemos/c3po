use super::super::compose::composeparser::ComposeParser;
use super::super::many::manyparser::ManyParser;
use crate::parser::parser::Parser;

/// Matches a `Parser` 1 or more times.
#[derive(Debug)]
pub struct SomeParser<TOutput: Send + Sync, TParser: Parser<Output = TOutput>> {
    pub(super) parser: ComposeParser<
        TOutput,
        TParser,
        Vec<TOutput>,
        ManyParser<TOutput, TParser>,
        Vec<TOutput>,
        fn(TOutput, Vec<TOutput>) -> Vec<TOutput>,
    >,
    pub(super) base_kind: String,
    pub(super) kind: String,
}

impl<TOutput: Send + Sync, TParser: Parser<Output = TOutput>> SomeParser<TOutput, TParser> {
    /// Creates a new `ManyParser` that matches the given parser 1 or more times.
    pub fn new(parser: TParser) -> SomeParser<TOutput, TParser> {
        let many = ManyParser::new(parser.clone());
        let kind = format!("some({})", parser.kind());
        let base_kind = parser.kind().to_string();

        let compose = ComposeParser::using_combiner(
            parser,
            many,
            (|f, mut v| {
                v.insert(0, f);
                v
            }) as fn(TOutput, Vec<TOutput>) -> Vec<TOutput>,
        );

        SomeParser {
            parser: compose,
            kind,
            base_kind,
        }
    }
}
