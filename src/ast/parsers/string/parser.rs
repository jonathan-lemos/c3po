use super::stringparser::StringParser;
use crate::immut_iter::immut_iterable::ImmutableIterable;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl Parser<char, String> for StringParser {
    fn parse<'a>(&self, mut cursor: Option<Cursor<'a, char>>) -> Parse<'a, char, String> {
        let mut char_iter = self.string().chars().peekable();

        while let Some(c) = cursor {
            match char_iter.peek() {
                Some(char) => {
                    if char != c.current() {
                        return Parse::failure(Some(c), format!("Expected '{}', got '{}'", c.current(), char));
                    }
                }
                None => break,
            }

            cursor = c.next_immut();
            char_iter.next();
        }

        if let Some(char) = char_iter.peek() {
            Parse::failure(None, format!("Expected '{}', but reached EOF before it could be read.", char))
        } else {
            Parse::success(cursor, self.string.clone())
        }
    }

    fn kind(&self) -> &str {
        &self.string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parses_abc() {
        let chars: Vec<char> = "abc".chars().collect();
        let cursor = Cursor::new(&chars);

        let parser = StringParser::new("abc");
        let result = parser.parse(cursor).unwrap();

        assert_eq!(result.next(), &None);
        assert_eq!(result.value(), &"abc");
    }
}
