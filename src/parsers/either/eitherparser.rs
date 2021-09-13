use crate::parser::parser::Parser;
use std::marker::PhantomData;

/// Tries to match one parser, then tries to match the other.
/// 
/// It tries to match the left parser first, and only runs the right if the left doesn't match.
#[derive(Debug)]
pub struct EitherParser<
    TLeftOutput,
    TLeft,
    TRightOutput,
    TRight,
    TFinalOutput,
    FLeftMapper,
    FRightMapper,
> where
    TLeftOutput: Send + Sync,
    TLeft: Parser<Output = TLeftOutput>,
    TRightOutput: Send + Sync,
    TRight: Parser<Output = TRightOutput>,
    TFinalOutput: Send + Sync,
    FLeftMapper: (Fn(TLeftOutput) -> TFinalOutput) + Send + Sync + Clone,
    FRightMapper: (Fn(TRightOutput) -> TFinalOutput) + Send + Sync + Clone,
{
    pub(super) left: TLeft,
    pub(super) right: TRight,
    pub(super) if_left: FLeftMapper,
    pub(super) if_right: FRightMapper,
    pub(super) kind: String,
    pub(super) _first_output_marker: PhantomData<TLeftOutput>,
    pub(super) _second_output_marker: PhantomData<TRightOutput>,
    pub(super) _final_output_marker: PhantomData<TFinalOutput>,
}

impl<TLeftOutput, TLeft, TRightOutput, TRight, TFinalOutput, FLeftMapper, FRightMapper>
    EitherParser<TLeftOutput, TLeft, TRightOutput, TRight, TFinalOutput, FLeftMapper, FRightMapper>
where
    TLeftOutput: Send + Sync,
    TLeft: Parser<Output = TLeftOutput>,
    TRightOutput: Send + Sync,
    TRight: Parser<Output = TRightOutput>,
    TFinalOutput: Send + Sync,
    FLeftMapper: (Fn(TLeftOutput) -> TFinalOutput) + Send + Sync + Clone,
    FRightMapper: (Fn(TRightOutput) -> TFinalOutput) + Send + Sync + Clone,
{
    /// Creates a new EitherParser which returns the value given from one of the given mappers.
    /// 
    /// # Arguments
    /// * `left`      - The first parser to run.
    /// * `right`     - The parser to run if `left` doesn't match.
    /// * `if_left`   - The mapping function to run if `left` matches.
    /// * `if_right`  - The mapping function to run if `right` matches.
    pub fn with_mappers(
        left: TLeft,
        right: TRight,
        if_left: FLeftMapper,
        if_right: FRightMapper,
    ) -> Self {
        let kind = format!("({}) or ({})", left.kind(), right.kind());

        Self {
            left,
            right,
            if_left,
            if_right,
            kind,
            _first_output_marker: PhantomData,
            _second_output_marker: PhantomData,
            _final_output_marker: PhantomData,
        }
    }
}

impl<TLeftOutput, TLeft, TRightOutput, TRight>
    EitherParser<
        TLeftOutput,
        TLeft,
        TRightOutput,
        TRight,
        Result<TLeftOutput, TRightOutput>,
        fn(TLeftOutput) -> Result<TLeftOutput, TRightOutput>,
        fn(TRightOutput) -> Result<TLeftOutput, TRightOutput>,
    >
where
    TLeftOutput: Send + Sync,
    TLeft: Parser<Output = TLeftOutput>,
    TRightOutput: Send + Sync,
    TRight: Parser<Output = TRightOutput>,
{
    /// Creates a new EitherParser which returns a `Result<TLeftOutput, TRightOutput>`.
    /// 
    /// The "error" case in the `Result` is not a true error, it only means that the right parser succeeded and the left one didn't.
    /// 
    /// # Arguments
    /// * `left`      - The first parser to run.
    /// * `right`     - The parser to run if `left` doesn't match.
    pub fn new(left: TLeft, right: TRight) -> Self {
        let kind = format!("({}) or ({})", left.kind(), right.kind());

        Self {
            left,
            right,
            if_left: Ok,
            if_right: Err,
            kind,
            _first_output_marker: PhantomData,
            _second_output_marker: PhantomData,
            _final_output_marker: PhantomData,
        }
    }
}
