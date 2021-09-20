use super::composeparser::ComposeParser;
use crate::parser::parser::Parser;
use std::marker::PhantomData;

impl<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner> Clone
    for ComposeParser<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
where
    TFirstOutput: Send + Sync,
    TFirst: Parser<Output = TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<Output = TSecondOutput>,
    TFinalOutput: Send + Sync,
    FCombiner: (Fn(TFirstOutput, TSecondOutput) -> TFinalOutput) + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        ComposeParser {
            first: self.first.clone(),
            second: self.second.clone(),
            combiner: self.combiner.clone(),
            kind: self.kind.clone(),
            _first_output_marker: PhantomData,
            _second_output_marker: PhantomData,
            _final_output_marker: PhantomData,
        }
    }
}
