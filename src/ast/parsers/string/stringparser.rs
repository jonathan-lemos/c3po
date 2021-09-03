#[derive(Debug, Clone)]
pub struct StringParser {
    pub(super) string: String,
    pub(super) kind: String
}

impl StringParser {
    pub fn new<I: Into<String>>(s: I) -> StringParser {
        let string = s.into();
        let kind = format!("'{}'", &string);        

        StringParser {
            string,
            kind
        }
    }
     
    pub fn string(&self) -> &str {
        &self.string
    }
}
