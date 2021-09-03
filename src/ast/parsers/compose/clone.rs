use super::composeparser::ComposeParser;
use crate::parser::parser::Parser;
use std::marker::PhantomData;

impl<TLexeme, TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner> Clone
    for ComposeParser<TLexeme, TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
where
    TLexeme: Send + Sync,
    TFirstOutput: Send + Sync,
    TFirst: Parser<TLexeme, TFirstOutput>,
    TSecondOutput: Send + Sync,
    TSecond: Parser<TLexeme, TSecondOutput>,
    TFinalOutput: Send + Sync,
    FCombiner: (Fn(TFirstOutput, TSecondOutput) -> TFinalOutput) + Send + Sync + Clone
{
    fn clone(&self) -> Self {
        ComposeParser {
            first: self.first.clone(),
            second: self.second.clone(),
            combiner: self.combiner.clone(),
            kind: self.kind.clone(),
            _lexeme_marker: PhantomData,
            _first_output_marker: PhantomData,
            _second_output_marker: PhantomData,
            _final_output_marker: PhantomData
        }
    }
}
