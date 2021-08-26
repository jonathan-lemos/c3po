use super::iter::cursoriterator::CursorIterator;
use std::cmp::min;

/// Represents a position within a sequence of lexemes.
/// 
/// A lexeme is a fundamental unit in your language e.g. words, punctuation for English, identifiers, keywords, punctuation for most programming languages.
/// Use `TLexeme = char` for a simple string parser.
/// 
/// Cursors should be immutable and cloneable so they can be safely reused across functions/threads.
/// 
pub trait Cursor: Clone + Send + Sync {
    type Lexeme: Send + Sync;

    /// The lexeme under the cursor.
    fn current(&self) -> &Self::Lexeme {
        &self.source()[self.pos()]
    }

    /// The current lexeme and all lexemes beyond.
    fn input(&self) -> &[Self::Lexeme] {
        let pos = min(self.pos(), self.source().len());
        &self.source()[pos..]
    }

    /// Yields this Cursor and all subsequent Cursors.
    fn iter(&self) -> CursorIterator<Self::Lexeme, Self> {
        CursorIterator::new(self)
    }

    /// A Cursor pointing to the next lexeme, or `None` if there isn't one.
    fn next_immut(&self) -> Option<Self>;

    /// The 0-based index of the cursor within the `source()`.
    fn pos(&self) -> usize;

    /// The sequence of lexemes that the cursor is pointing to.
    fn source(&self) -> &[Self::Lexeme];
}
