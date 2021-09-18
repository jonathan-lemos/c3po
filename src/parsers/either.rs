use crate::immut_iter::immut_iterable::ImmutableIterable;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

pub fn either<TOutput1, TState1, FMapper1, TOutput2, TState2, FMapper2, TFinalOutput>(
    first: Parser<TOutput1, TState1>,
    first_mapper: FMapper1,
    second: Parser<TOutput2, TState1>,
    second_mapper: FMapper2,
) -> Parser<TFinalOutput>
where
    FMapper1: FnOnce(TOutput1) -> TFinalOutput,
    FMapper2: FnOnce(TOutput2) -> TFinalOutput,
{
    let kind = format!("({}) or ({})", first.kind, second.kind);
    Parser::stateless(kind, move |cursor, state| {
        first.parse(cursor)
            .or_else(|s| second.parse(cursor).map_reason(|r| s.))
    })
}
