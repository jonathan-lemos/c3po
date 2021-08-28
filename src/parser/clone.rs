use super::parser::Parser;
use super::cursor::cursor::Cursor;

impl<TLexeme, TCursor: Cursor<Lexeme = TLexeme>, TOutput> Clone for Parser<'_, TLexeme, TCursor, TOutput> {
    fn clone(&self) -> Self {
        Parser {
            func: Box::new(self.func),
            kind: self.kind.clone()
        }
    }
}
