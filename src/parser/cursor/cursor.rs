use std::ptr;

/// Represents a position in a string.
#[derive(Debug)]
pub struct Cursor<'a> {
    pub(super) sequence: &'a str,
    pub(super) pos_bytes: usize,
    pub(super) pos_chars: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new Cursor at the beginning of a string.
    pub fn new(s: &'a str) -> Option<Self> {
        if s.len() == 0 {
            None
        } else {
            Some(Self {
                sequence: s,
                pos_bytes: 0,
                pos_chars: 0,
            })
        }
    }

    /// Gets the character that this Cursor is pointing to.
    pub fn current(&self) -> char {
        self.current_str()
            .chars()
            .next()
            .expect("Cursor is pointing to an invalid location. This should never happen.")
    }

    /// Gets the string slice that this Cursor is pointing to, starting from the byte that this Cursor points to.
    pub fn current_str(&self) -> &'a str {
        &self.sequence[self.pos_bytes..]
    }

    /// Gets the amount of `next_immut()` calls needed to reach one Cursor from the other.
    ///
    /// Returns `None` if the two cursors have different sources.
    pub fn difference(&self, other: &Self) -> Option<usize> {
        if ptr::eq(self.sequence, other.sequence) {
            Some(if self.pos_chars >= other.pos_chars {
                self.pos_chars - other.pos_chars
            } else {
                other.pos_chars - self.pos_chars
            })
        } else {
            None
        }
    }

    /// Gets the 0-based position of the cursor within the `source()` in characters.
    pub fn pos(&self) -> usize {
        self.pos_chars
    }

    /// Gets the source string slice that this Cursor points to, including the characters behind this Cursor.
    pub fn source(&self) -> &'a str {
        &self.sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_creates_on_not_empty() {
        assert!(Cursor::new("a").is_some())
    }

    #[test]
    fn cursor_doesnt_create_on_empty() {
        assert!(Cursor::new("").is_none())
    }
}
