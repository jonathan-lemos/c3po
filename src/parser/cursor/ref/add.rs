use num_traits::Num;
use std::ops::Add;
use super::position::Position;
use std::cmp::min;

impl<TLexeme> Add<usize> for Position<'_, TLexeme> {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        let new_pos = self.pos + min(rhs, self.input.len());

        Position {
            input: &self.input[new_pos..],
            source: self.source,
            pos: new_pos
        }
    }
}
