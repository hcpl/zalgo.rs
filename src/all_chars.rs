#[cfg(any(feature = "nightly", stable_fused_iterator))]
use std::iter;

use {UP_CHARS, MIDDLE_CHARS, DOWN_CHARS};


/// Returns an iterator of combined kinds of Zalgo `char`s. These are all of the
/// `char`s used to create a generated Zalgo `String`.
///
/// # Examples
///
/// A basic usage:
///
/// ```rust
/// let _ = zalgo::all_chars();
///
/// // You can then manually use this iterator for your own uses.
/// ```
pub fn all_chars() -> AllChars {
    AllChars {
        pos: 0,
        pos_back: UP_CHARS.len() + MIDDLE_CHARS.len() + DOWN_CHARS.len(),
    }
}

/// An iterator of combined kinds of Zalgo `char`s.
///
/// This struct is created by the [`all_chars`] function. See its documentation
/// for more details.
///
/// [`all_chars`]: fn.all_chars.html
pub struct AllChars {
    pos: usize,
    pos_back: usize,
}

// Optimized implementations of various iterator-related methods

impl Iterator for AllChars {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.nth(0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn last(mut self) -> Option<char> {
        self.next_back()
    }

    fn nth(&mut self, n: usize) -> Option<char> {
        let remaining = self.len();

        if n >= remaining {
            self.pos += remaining;
            return None;
        }

        let mut pos = self.pos + n;

        if pos < UP_CHARS.len() {
            self.pos += n + 1;
            return Some(UP_CHARS[pos]);
        }

        pos -= UP_CHARS.len();
        if pos < MIDDLE_CHARS.len() {
            self.pos += n + 1;
            return Some(MIDDLE_CHARS[pos]);
        }

        pos -= MIDDLE_CHARS.len();
        if pos < DOWN_CHARS.len() {
            self.pos += n + 1;
            return Some(DOWN_CHARS[pos]);
        }

        unreachable!();
    }
}

impl ExactSizeIterator for AllChars {
    fn len(&self) -> usize {
        debug_assert!(self.pos <= self.pos_back);
        self.pos_back - self.pos
    }

    #[cfg(feature = "nightly")]
    fn is_empty(&self) -> bool {
        debug_assert!(self.pos <= self.pos_back);
        self.pos == self.pos_back
    }
}

impl DoubleEndedIterator for AllChars {
    fn next_back(&mut self) -> Option<char> {
        let remaining = self.len();

        if remaining == 0 {
            return None;
        }

        let mut pos_back = self.pos_back - 1;

        if pos_back < UP_CHARS.len() {
            self.pos_back -= 1;
            return Some(UP_CHARS[pos_back]);
        }

        pos_back -= UP_CHARS.len();
        if pos_back < MIDDLE_CHARS.len() {
            self.pos_back -= 1;
            return Some(MIDDLE_CHARS[pos_back]);
        }

        pos_back -= MIDDLE_CHARS.len();
        if pos_back < DOWN_CHARS.len() {
            self.pos_back -= 1;
            return Some(DOWN_CHARS[pos_back]);
        }

        unreachable!();
    }
}

#[cfg(feature = "nightly")]
unsafe impl iter::TrustedLen for AllChars {}

#[cfg(any(feature = "nightly", stable_fused_iterator))]
impl iter::FusedIterator for AllChars {}
