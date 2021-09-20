use fancy_regex::Regex;
use std::str::FromStr;

/// Matches the input using a regular expression.
#[derive(Debug, Clone)]
pub struct RegexParser {
    pub(super) regex: Regex,
    pub(super) kind: String,
}

impl RegexParser {
    /// Creates a new `RegexParser` out of the given string.
    ///
    /// # Arguments
    /// * `s` - A `fancy_regex` style regular expression. Performance is improved if this regex starts with `^`, although this is not required for this parser to function correctly.
    pub fn new<S: Into<String>>(s: S) -> RegexParser {
        let pattern = s.into();

        let regex =
            Regex::from_str(&pattern).expect(&format!("The regex '{}' is invalid.", pattern));

        let kind = format!("regex({})", pattern);

        RegexParser { regex, kind }
    }
}
