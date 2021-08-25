use super::parser::Parser;

impl<T> Parser<T> {
    fn parse(&self) -> Option<T> {
        (*self.func)()
    }
}