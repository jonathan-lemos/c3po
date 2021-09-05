pub mod ast;
pub mod parser;
pub mod immut_iter;
pub mod lexer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
