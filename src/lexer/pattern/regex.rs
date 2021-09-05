use crate::lexer::pattern::pattern::Pattern;
use fancy_regex::Regex;

impl Pattern for Regex {
    fn match_start(&self, s: &str) -> Option<usize> {
        match self
            .find(s)
            .expect(format!("{} is not a valid regex.", self))
        {
            Some(s) => {
                if s.start() == 0 {
                    Some(s.end())
                } else {
                    None
                }
            }

            None => None,
        }
    }
}
