use crate::parser::parse::parse::Parse;
use crate::parser::parser::Parser;
use crate::immut_iter::immut_iterable::ImmutableIterable;

pub fn string<S: Into<String>>(string: S) -> Parser<String> {
    let string = string.into();
    let kind = format!("\"{}\"", string);

    Parser::stateless(kind, Box::new(move |mut cursor| {
        let mut char_iter = string.chars().peekable();

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
            Parse::success_stateless(cursor, string.clone())
        }
    }))
}
