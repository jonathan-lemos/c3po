use super::pattern::Pattern;

impl Pattern for String {
    fn match_start(&self, s: &str) -> Option<usize> {
        if s.starts_with(self) {
            Some(self.len())
        } else {
            None
        }
    }
}

impl Pattern for &str {
    fn match_start(&self, s: &str) -> Option<usize> {
        if s.starts_with(self) {
            Some(self.len())
        } else {
            None
        }
    }
}
