use crate::parser::parser::Parser;
use std::marker::PhantomData;

/// Composes two parsers, matching the first parser and then the second parser, returning both results.
#[derive(Debug)]
pub struct ComposeParser<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
where
    TFirstOutput: Send + Sync,
    TFirst: Parser<Output = TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<Output = TSecondOutput>,
    TFinalOutput: Send + Sync,
    FCombiner: (Fn(TFirstOutput, TSecondOutput) -> TFinalOutput) + Send + Sync + Clone
{
    pub(super) first: TFirst,
    pub(super) second: TSecond,
    pub(super) combiner: FCombiner,
    pub(super) kind: String,
    pub(super) _first_output_marker: PhantomData<TFirstOutput>,
    pub(super) _second_output_marker: PhantomData<TSecondOutput>,
    pub(super) _final_output_marker: PhantomData<TFinalOutput>,
}

impl<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
    ComposeParser<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
where
    TFirstOutput: Send + Sync,
    TFirst: Parser<Output = TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<Output = TSecondOutput>,
    TFinalOutput: Send + Sync,
    FCombiner: (Fn(TFirstOutput, TSecondOutput) -> TFinalOutput) + Clone + Send + Sync
{
    /// Creates a new ComposeParser that combines the two results with a function.
    /// 
    /// # Arguments
    /// * `first`    - The first parser to run.
    /// * `second`   - The parser to run after `first`.
    /// * `combiner` - A function that combines the results of the two parsers.
    pub fn using_combiner(
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
            _first_output_marker: PhantomData,
            _second_output_marker: PhantomData,
            _final_output_marker: PhantomData
        }
    }
}

impl<TFirstOutput, TFirst, TSecondOutput, TSecond>
    ComposeParser<TFirstOutput, TFirst, TSecondOutput, TSecond, (TFirstOutput, TSecondOutput), fn(TFirstOutput, TSecondOutput) -> (TFirstOutput, TSecondOutput)>
where
    TFirstOutput: Send + Sync,
    TFirst: Parser<Output = TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<Output = TSecondOutput>,
{
    /// Creates a new ComposeParser that combines the two results into a tuple.
    /// 
    /// # Arguments
    /// * `first`  - The first parser to run.
    /// * `second` - The parser to run after `first`.
    pub fn new(
        first: TFirst,
        second: TSecond
    ) -> Self {
        Self::using_combiner(first, second, |a, b| (a, b))
    }
}
