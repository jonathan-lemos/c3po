use super::stringparser::StringParser;
use crate::immut_iter::immut_iterable::ImmutableIterable;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl Parser<String> for StringParser {
    fn parse<'a>(&self, mut cursor: Option<Cursor<'a>>) -> Parse<'a, String> {
        let mut char_iter = self.string().chars().peekable();

        while let Some(c) = cursor {
            match char_iter.peek() {
                Some(char) => {
                    if char != &c.current() {
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
        let cursor = Cursor::new("abc");

        let parser = StringParser::new("abc");
        let result = parser.parse(cursor).unwrap();

        assert_eq!(result.next(), &None);
        assert_eq!(result.value(), &"abc");
    }

    #[test]
    pub fn points_to_right_char_after_parse() {
        let cursor = Cursor::new("abcd");

        let parser = StringParser::new("abc");
        let result = parser.parse(cursor).unwrap();

        assert_eq!(result.next().unwrap().current(), 'd');
        assert_eq!(result.value(), &"abc");
    }

    #[test]
    pub fn does_not_parse_if_string_doesnt_match() {
        let cursor = Cursor::new("xabc");

        let parser = StringParser::new("abc");
        let result = parser.parse(cursor);

        assert!(result.is_failure())
    }
}
