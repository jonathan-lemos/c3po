use super::parser::Parser;

impl<'a, TLexeme, TOutput> Clone for Parser<'a, TLexeme, TOutput> {
    fn clone(&self) -> Self {
        Parser {
            func: self.func,
            kind: self.kind.clone()
        }
    }
}
