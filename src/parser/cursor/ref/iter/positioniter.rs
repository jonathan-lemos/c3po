use super::super::position::Position;

pub struct PositionIter<'a, TLexeme> {
    pos: Position<'a, TLexeme>
}

impl<TLexeme> PositionIter<'_, TLexeme> {
    pub fn new(slice: &[TLexeme]) {
        
    }
}
