use std::cmp::Ordering;
use std::cmp::min;
use std::ptr;

/// Represents a position within a sequence of lexemes.
/// 
/// A lexeme is a fundamental unit in your language e.g. words, punctuation for English, identifiers, keywords, punctuation for most programming languages.
/// Use `TLexeme = char` for a simple string parser.
/// 
/// Cursors should be immutable and cloneable so they can be safely reused across functions/threads.
///
/// The `Iterator::next()` implementation should return the cursor pointing to the next lexeme, or None if there are no more lexemes.
/// 
/// The default `Eq` implementation checks that the `pos()` values are the same, and then checks that the `source()` slices point to the same memory.
/// The default `PartialOrd` implementation returns `None` if the `source()` slices point to different memory, and otherwise compares the two `pos()` values.
/// 
pub trait Cursor: Clone + Iterator<Item = Self> + Send + Sync {
    type Lexeme: Send + Sync;

    /// The lexeme under the cursor.
    fn current(&self) -> &Self::Lexeme {
        &self.source()[self.pos()]
    }

    fn eq(&self, other: &Self) -> bool {
        self.pos() == other.pos() && ptr::eq(self.source(), other.source())
    }

    /// The current lexeme and all lexemes beyond.
    fn input(&self) -> &[Self::Lexeme] {
        let pos = min(self.pos(), self.source().len());
        &self.source()[pos..]
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !ptr::eq(self.source(), other.source()) {
            None
        } else {
            PartialOrd::partial_cmp(&self.pos(), &other.pos())
        }
    }

    /// The 0-based index of the cursor.
    fn pos(&self) -> usize;

    /// The sequence of lexemes that the cursor is pointing to.
    fn source(&self) -> &[Self::Lexeme];
}
