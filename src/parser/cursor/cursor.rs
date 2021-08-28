use super::iter::cursoriterator::CursorIterator;
use std::cmp::min;
use std::iter::FromIterator;

/// Represents a position within a sequence of lexemes.
///
/// A lexeme is a fundamental unit in your language e.g. words, punctuation for English, identifiers, keywords, punctuation for most programming languages.
/// Use `TLexeme = char` for a simple string parser.
///
/// Cursors should be immutable and cloneable so they can be safely reused across functions/threads.
///
pub trait Cursor: Clone + Send + Sync {
    type Lexeme: Send + Sync;

    /// A Cursor pointing to the next lexeme, or `None` if there isn't one.
    fn next_immut(&self) -> Option<Self>;

    /// The 0-based index of the cursor within the `source()`.
    fn pos(&self) -> usize;

    /// The sequence of lexemes that the cursor is pointing to.
    fn source(&self) -> &[Self::Lexeme];

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

    /// A `Vec` of exactly `n` cursors including this one, or `None` if `remaining() < n`.
    fn require_n(&self, n: usize) -> Option<Vec<Self>> {
        let cursors: Vec<Self> = self.up_to_n(n);

        if cursors.len() != n {
            None
        } else {
            Some(cursors)
        }
    }

    /// How many lexemes are there from this Cursor `pos()` and beyond?
    fn remaining(&self) -> usize {
        self.source().len() - self.pos()
    }

    /// A `FromIterator` of `min(n, remaining())` Cursors including this one.
    fn up_to_n<B: FromIterator<Self>>(&self, n: usize) -> B {
        let cursors = (0..n - 1).scan(Some(self.clone()), |a, _| {
            *a = match a {
                None => None,
                Some(s) => s.next_immut(),
            };

            match a {
                None => None,
                Some(s) => s.next_immut(),
            }
        });

        cursors.collect()
    }
}
