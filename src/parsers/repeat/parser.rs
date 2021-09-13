use super::repeatparser::RepeatParser;
use crate::parser::cursor::cursor::Cursor;
use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;

impl<TOutput, TParser> Parser for RepeatParser<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<Output = TOutput>,
{
    type Output = Vec<TOutput>;

    fn parse<'a>(&self, mut cursor: Option<Cursor<'a>>) -> Parse<'a, Vec<TOutput>> {
        let (lb, ub) = match self.bounds {
            Some(s) => s,
            None => return Parse::failure(cursor, "This parser cannot parse anything because its range has no valid values.")
        };

        let mut results = Vec::new();

        while let Some(c) = cursor {
            if ub.clone().map(|n| results.len() == n).unwrap_or(false) {
                break;
            }

            let parse = self.parser.parse(Some(c));
            if let Parse::Success(success) = parse {
                cursor = *success.next();
                results.push(success.into_value());
            } else {
                break;
            }
        }

        if results.len() < lb {
            Parse::failure(cursor, format!("Expected at least {} values, but was only able to parse {}", lb, results.len()))
        } else {
            Parse::success(cursor, results)
        }
    }

    fn kind(&self) -> &str {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::string::stringparser::StringParser;
    use super::*;

    #[test]
    fn parses_lower_bound() {
        let cursor = Cursor::new("abcabcabc");

        let sp = StringParser::new("abc");
        let parser = RepeatParser::range(sp, 3..=5);

        let result = parser.parse(cursor).unwrap();
        let values = result.value();

        assert_eq!(result.next(), &None);
        assert_eq!(values.len(), 3);
        assert!(values.iter().all(|x| x == "abc"));
    }

    #[test]
    fn parses_upper_bound() {
        let cursor = Cursor::new("abcabcabcabcabc");

        let sp = StringParser::new("abc");
        let parser = RepeatParser::range(sp, 3..=5);

        let result = parser.parse(cursor).unwrap();
        let values = result.value();

        assert_eq!(result.next(), &None);
        assert_eq!(values.len(), 5);
        assert!(values.iter().all(|x| x == "abc"));
    }

    #[test]
    fn parses_exact() {
        let cursor = Cursor::new("abcabcabc");

        let sp = StringParser::new("abc");
        let parser = RepeatParser::range(sp, 3..=3);

        let result = parser.parse(cursor).unwrap();
        let values = result.value();

        assert_eq!(result.next(), &None);
        assert_eq!(values.len(), 3);
        assert!(values.iter().all(|x| x == "abc"));
    }

    #[test]
    fn parses_without_exhausting_input() {
        let cursor = Cursor::new("abcabcabcabcabc");

        let sp = StringParser::new("abc");
        let parser = RepeatParser::range(sp, 1..3);

        let result = parser.parse(cursor).unwrap();
        let values = result.value();

        assert_eq!(result.next().unwrap().current(), 'a');
        assert_eq!(values.len(), 2);
        assert!(values.iter().all(|x| x == "abc"));
    }

    #[test]
    fn fails_if_not_enough() {
        let cursor = Cursor::new("abc");

        let sp = StringParser::new("abc");
        let parser = RepeatParser::range(sp, 2..);

        let result = parser.parse(cursor);

        assert!(result.is_failure());
    }
}
