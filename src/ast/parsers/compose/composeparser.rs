use crate::parser::parser::Parser;
use std::marker::PhantomData;

/// Composes two parsers, running the first parser and then the second parser, returning both results.
pub struct ComposeParser<TLexeme, TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
where
    TLexeme: Send + Sync,
    TFirstOutput: Send + Sync,
    TFirst: Parser<TLexeme, TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<TLexeme, TSecondOutput>,
    TFinalOutput: Send + Sync,
    FCombiner: (Fn(TFirstOutput, TSecondOutput) -> TFinalOutput) + Send + Sync + Clone
{
    pub(super) first: TFirst,
    pub(super) second: TSecond,
    pub(super) combiner: FCombiner,
    pub(super) kind: String,
    pub(super) _lexeme_marker: PhantomData<TLexeme>,
    pub(super) _first_output_marker: PhantomData<TFirstOutput>,
    pub(super) _second_output_marker: PhantomData<TSecondOutput>,
    pub(super) _final_output_marker: PhantomData<TFinalOutput>,
}

impl<TLexeme, TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
    ComposeParser<TLexeme, TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
where
    TLexeme: Send + Sync,
    TFirstOutput: Send + Sync,
    TFirst: Parser<TLexeme, TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<TLexeme, TSecondOutput>,
    TFinalOutput: Send + Sync,
    FCombiner: (Fn(TFirstOutput, TSecondOutput) -> TFinalOutput) + Clone + Send + Sync
{
    pub fn with_combiner(
        first: TFirst,
        second: TSecond,
        combiner: FCombiner,
    ) -> Self {
        let kind = format!("{} + {}", first.kind(), second.kind());

        ComposeParser {
            first,
            second,
            combiner,
            kind,
            _lexeme_marker: PhantomData,
            _first_output_marker: PhantomData,
            _second_output_marker: PhantomData,
            _final_output_marker: PhantomData
        }
    }
}

impl<TLexeme, TFirstOutput, TFirst, TSecondOutput, TSecond>
    ComposeParser<TLexeme, TFirstOutput, TFirst, TSecondOutput, TSecond, (TFirstOutput, TSecondOutput), fn(TFirstOutput, TSecondOutput) -> (TFirstOutput, TSecondOutput)>
where
    TLexeme: Send + Sync,
    TFirstOutput: Send + Sync,
    TFirst: Parser<TLexeme, TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<TLexeme, TSecondOutput>,
{
    pub fn new(
        first: TFirst,
        second: TSecond
    ) -> Self {
        Self::with_combiner(first, second, |a, b| (a, b))
    }
}
