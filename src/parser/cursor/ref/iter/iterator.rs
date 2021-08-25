impl<'a, TLexeme> Iterator for PositionIter<'a, TLexeme> {
    type Item = Position<'a, TLexeme>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.input.len() == 0 {
            None
        } else {
            let p = self.pos;
            self.pos = self.pos + 1;
            Some(p)
        }
    }
}
