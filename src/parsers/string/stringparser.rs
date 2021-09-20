/// A parser that matches a string.
#[derive(Debug, Clone)]
pub struct StringParser {
    pub(super) string: String,
    pub(super) kind: String,
}

impl StringParser {
    /// Creates a new `StringParser` out of the given string.
    pub fn new<I: Into<String>>(s: I) -> StringParser {
        let string = s.into();
        let kind = format!("'{}'", &string);

        StringParser { string, kind }
    }

    /// The string that this parser is matching.
    pub fn string(&self) -> &str {
        &self.string
    }
}
