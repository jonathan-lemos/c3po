use super::parser::Parser;
use std::ops::BitOr;

impl<TOutput, TState> BitOr for Parser<TOutput, TState> {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let kind = format!("({}) or ({})", self.kind(), other.kind());

        Parser::new(kind, self.state, Box::new(move |cursor, state| {

        }))
    }
}
