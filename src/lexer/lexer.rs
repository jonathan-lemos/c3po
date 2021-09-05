use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    literals: HashSet<&'a str>,
    position: usize,
}
