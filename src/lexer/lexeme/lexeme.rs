#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexeme {
    kind: String,
    value: String
}

impl Lexeme {
    pub fn new<A: Into<String>, B: Into<String>>(kind: A, value: B) -> Self {
        Lexeme {
            kind: kind.into(),
            value: value.into()
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
