use super::regexparser::RegexParser;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl Parser for RegexParser {
    type Output = String;

    fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, String> {
        let slice = match cursor {
            Some(s) => s.current_str(),
            None => ""
        };

        let mat = match self.regex.find(slice) {
            Ok(Some(s)) => s,
            _ => return Parse::failure(cursor, "The regex didn't match.")
        };

        if mat.start() != 0 {
            Parse::failure(cursor, "The regex did not match the beginning of the string.")
        } else {
            let ret = mat.as_str().to_string();
            Parse::success(cursor.and_then(|c| c + ret.chars().count()), ret)
        }

    }

    fn kind(&self) -> &str {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parses_abc() {
        let cursor = Cursor::new("abc");

        let parser = RegexParser::new("[a-z]+");
        let result = parser.parse(cursor).unwrap();

        assert_eq!(result.next(), &None);
        assert_eq!(result.value(), &"abc");
    }

    #[test]
    pub fn points_to_right_char_after_parse() {
        let cursor = Cursor::new("abc9");

        let parser = RegexParser::new("[a-z]+");
        let result = parser.parse(cursor).unwrap();

        assert_eq!(result.next().unwrap().current(), '9');
        assert_eq!(result.value(), &"abc");
    }

    #[test]
    pub fn does_not_parse_if_string_doesnt_match() {
        let cursor = Cursor::new("xabc");

        let parser = RegexParser::new("[a-c]+");
        let result = parser.parse(cursor);

        assert!(result.is_failure())
    }
}
