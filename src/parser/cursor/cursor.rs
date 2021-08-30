use std::ptr;

/// Represents a position in a slice of lexemes.
#[derive(Debug, Clone, Copy)]
pub struct Cursor<'a, TLexeme> {
    pub(super) source: &'a [TLexeme],
    pub(super) pos: usize,
}

impl<'a, TLexeme> Cursor<'a, TLexeme> {
    /// Creates a new Cursor at the beginning of a slice of lexemes.
    ///
    /// Returns `None` if the input slice is empty.
    pub fn new(lexemes: &'a [TLexeme]) -> Option<Self> {
        if lexemes.len() == 0 {
            None
        } else {
            Some(Self {
                source: lexemes,
                pos: 0,
            })
        }
    }

    /// Gets the lexeme that this Cursor is pointing to.
    pub fn current(&self) -> &TLexeme {
        &self.source[self.pos]
    }

    /// Gets the amount of `next_immut()` calls needed to reach one Cursor from the other.
    /// 
    /// Returns `None` if the two cursors have different sources.
    pub fn difference(&self, other: &Self) -> Option<usize> {
        if ptr::eq(self.source, other.source) {
            Some(if self.pos >= other.pos {
                self.pos - other.pos
            } else {
                other.pos - self.pos
            })
        } else {
            None
        }
    }

    /// Creates the Cursor pointing to the next lexeme if there is one.
    pub fn next_immut(&'a self) -> Option<Self> {
        self + 1
    }

    /// Gets the 0-based position of the cursor within the `source()`.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Gets the source slice that the cursor is pointing to.
    pub fn source(&self) -> &[TLexeme] {
        self.source
    }
}
