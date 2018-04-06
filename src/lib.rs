// ISC License (ISC)
//
// Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
// RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
// CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// What is Zalgo?
//
//
// | Zalgo is an Internet legend about an ominous entity believed to cause
// | insanity, death and destruction of the world, similar to the creature Cthulhu
// | created by H.P. Lovecraft in the 1920s. Zalgo is often associated with
// | scrambled text on webpages and photos of people whose eyes and mouth have been
// | covered in black.
// |
// | - [icannwiki](http://icannwiki.com/GTLD)
//! A library for easily creating modifications of text with Zalgo characters.
//!
//! What is Zalgo?
//!
//!
//! > Zalgo is an Internet legend about an ominous entity believed to cause
//! > insanity, death and destruction of the world, similar to the creature Cthulhu
//! > created by H.P. Lovecraft in the 1920s. Zalgo is often associated with
//! > scrambled text on webpages and photos of people whose eyes and mouth have been
//! > covered in black.
//! >
//! > -- [knowyourmeme](http://knowyourmeme.com/memes/zalgo)
//!
//! An example to create a modified string with Zalgo text only above the string
//! with a high amount of Zalgo text is:
//!
//! ```rust
//! use zalgo::{ZalgoKind, ZalgoSize};
//!
//! let result = zalgo::gen("my string", ZalgoKind::UP, ZalgoSize::Maxi);
//! ```

#[macro_use]
extern crate bitflags;
extern crate rand;

/// Denotes characters to be used in the "upper" part of text.
pub static ZALGO_UP: [char; 50] = [
    '̍', '̎', '̄', '̅', '̿', '̑', '̆', '̐', '͒', '͗', '͑', '̇', '̈', '̊',
    '͂', '̓', '̈́', '͊', '͋', '͌', '̃', '̂', '̌', '͐', '̀', '́', '̋', '̏',
    '̒', '̓', '̔', '̽', '̉', 'ͣ', 'ͤ', 'ͥ', 'ͦ', 'ͧ', 'ͨ', 'ͩ', 'ͪ', 'ͫ',
    'ͬ', 'ͭ', 'ͮ', 'ͯ', '̾', '͛', '͆', '̚',
];
/// Denotes characters to be used in the "middle" part of text.
pub static ZALGO_MIDDLE: [char; 23] = [
    '̕', '̛', '̀', '́', '͘', '̡', '̢', '̧', '̨', '̴', '̵', '̶', '͏', '͜',
    '͝', '͞', '͟', '͠', '͢', '̸', '̷', '͡', '҉',
];
/// Denotes characters to be used in the "bottom" part of text.
pub static ZALGO_DOWN: [char; 40] = [
    '̖', '̗', '̘', '̙', '̜', '̝', '̞', '̟', '̠', '̤', '̥', '̦', '̩', '̪',
    '̫', '̬', '̭', '̮', '̯', '̰', '̱', '̲', '̳', '̹', '̺', '̻', '̼', 'ͅ',
    '͇', '͈', '͉', '͍', '͎', '͓', '͔', '͕', '͖', '͙', '͚', '̣',
];

use rand::{thread_rng, Rng};

bitflags! {
    /// A definition of the character type to be used for retrieval.
    pub struct ZalgoKind: u8 {
        /// Denotes characters to be used to appear in the top of the resulting
        /// string.
        const UP     = 0b00000001;
        /// Denotes characters to be used to appear in the middle of the resulting
        /// string (i.e. similar to strikethrough text).
        const MIDDLE = 0b00000010;
        /// Denotes characters to be used to appear in the bottom of the resulting
        /// string (i.e. similar to underlined text).
        const DOWN   = 0b00000100;
    }
}

/// The size of the Zalgo text within the string to produce.
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum ZalgoSize {
    /// Produce a larger amount of Zalgo text.
    Maxi,
    /// Produce a smaller amount of Zalgo text.
    Mini,
    /// Produce a randomized amount of Zalgo text.
    None,
}

/// Produces a `Vec` of the combined kinds of Zalgo `char`s. This is all of the
/// `char`s used to create a generated Zalgo `String`.
///
/// # Examples
///
/// A basic usage:
///
/// ```rust
/// let _ = zalgo::all();
///
/// // You can then manually use this `Vec` for your own uses.
/// ```
pub fn all() -> Vec<char> {
    let mut v = vec![];

    v.extend(ZALGO_UP.iter());
    v.extend(ZALGO_MIDDLE.iter());
    v.extend(ZALGO_DOWN.iter());

    v
}

/// Generates a String containing Zalgo text. This is customizable via defining
/// whether to include Zalgo text above the given string, in the middle of it,
/// and below it.
///
/// The amount of Zalgo text can be (more or less) defined by the value of the
/// `size` given. Read on the `ZalgoSize` for more information.
///
/// # Examples
///
/// Create Zalgo text with Zalgo `char`s in all positions, with a maximum amount
/// of Zalgo:
///
/// ```rust
/// use zalgo::{ZalgoKind, ZalgoSize};
///
/// let _ = zalgo::gen("test", ZalgoKind::all(), ZalgoSize::Maxi);
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the middle and lower positions,
/// with a minimum amount of Zalgo:
///
/// ```rust
/// use zalgo::{ZalgoKind, ZalgoSize};
///
/// let _ = zalgo::gen("test", ZalgoKind::MIDDLE | ZalgoKind::DOWN, ZalgoSize::Mini);
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the lower position, with a
/// random amount of Zalgo (can be a low amount or high amount):
///
/// ```rust
/// use zalgo::{ZalgoKind, ZalgoSize};
///
/// let _ = zalgo::gen("test", ZalgoKind::DOWN, ZalgoSize::None);
/// ```
///
/// Consequentially, you can also not modify your given text with any Zalgo:
///
/// ```rust
/// use zalgo::{ZalgoKind, ZalgoSize};
///
/// let _ = zalgo::gen("test", ZalgoKind::empty(), ZalgoSize::None);
/// // Technically the `ZalgoSize` value given does not matter here.
/// ```
pub fn gen<S: Into<String>>(text: S, kind: ZalgoKind, size: ZalgoSize) -> String {
    let val = text.into();

    // The base String where the original text and new Zalgo text will be
    // appended to.
    let mut result = String::new();

    let mut rng = thread_rng();

    for ch in val.chars() {
        // Skip the text if it's already a Zalgo char
        if is_zalgo(ch) {
            continue;
        }

        // Push the given character to the resultant string no matter what
        result.push(ch);

        let count_up = rng.gen_range(0, ZALGO_UP.len());
        let count_mid = if size == ZalgoSize::None {
            rng.gen_range(0, ZALGO_MIDDLE.len()) / 2
        } else {
            rng.gen_range(0, ZALGO_MIDDLE.len())
        };
        let count_down = rng.gen_range(0, ZALGO_DOWN.len());

        if kind.contains(ZalgoKind::UP) {
            for _ in 0..count_up {
                let get = rng.gen_range(0, count_up);
                result.push(ZALGO_UP[get]);
            }
        }

        if kind.contains(ZalgoKind::MIDDLE) {
            for _ in 0..count_mid {
                let get = rng.gen_range(0, count_mid);
                result.push(ZALGO_MIDDLE[get]);
            }
        }

        if kind.contains(ZalgoKind::DOWN) {
            for _ in 0..count_down {
                let get = rng.gen_range(0, count_down);
                result.push(ZALGO_DOWN[get]);
            }
        }
    }

    result
}

/// Determines whether a given `char` is a `Zalgo` `char`. This is checked by
/// checking if a combination of the defined Zalgo `char`s contains the given
/// `char`.
///
/// # Examples
///
/// A basic check:
///
/// ```rust
/// assert!(zalgo::is_zalgo('҉'));
///
/// // The following is simply a latin letter, and is not zalgo:
/// assert!(!zalgo::is_zalgo('a'));
/// ```
pub fn is_zalgo(ch: char) -> bool {
    all().contains(&ch)
}
