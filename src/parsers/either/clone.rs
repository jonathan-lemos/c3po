use std::marker::PhantomData;
use super::eitherparser::EitherParser;
use crate::parser::parser::Parser;

impl<TLeftOutput, TLeft, TRightOutput, TRight, TFinalOutput, FLeftMapper, FRightMapper> Clone
    for EitherParser<
        TLeftOutput,
        TLeft,
        TRightOutput,
        TRight,
        TFinalOutput,
        FLeftMapper,
        FRightMapper,
    >
where
    TLeftOutput: Send + Sync,
    TLeft: Parser<TLeftOutput>,
    TRightOutput: Send + Sync,
    TRight: Parser<TRightOutput>,
    TFinalOutput: Send + Sync,
    FLeftMapper: (Fn(TLeftOutput) -> TFinalOutput) + Send + Sync + Clone,
    FRightMapper: (Fn(TRightOutput) -> TFinalOutput) + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            if_left: self.if_left.clone(),
            if_right: self.if_right.clone(),
            kind: self.kind.clone(),
            _first_output_marker: PhantomData,
            _second_output_marker: PhantomData,
            _final_output_marker: PhantomData
        }
    }
}
