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
//! use zalgo::ZalgoSize;
//!
//! let result = zalgo::gen("my string", true, false, false, ZalgoSize::Maxi);
//! ```

extern crate rand;

use rand::{thread_rng, Rng};

/// A definition of the character type to be used for retrieval.
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum ZalgoKind {
    /// Denotes characters to be used to appear in the top of the resulting
    /// string.
    Up,
    /// Denotes characters to be used to appear in the middle of the resulting
    /// string (i.e. similar to strikethrough text).
    Middle,
    /// Denotes characters to be used to appear in the bottom of the resulting
    /// string (i.e. similar to underlined text).
    Down,
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
pub fn all<'a>() -> Vec<char> {
    let mut v = vec![];

    v.extend(chars(ZalgoKind::Up));
    v.extend(chars(ZalgoKind::Middle));
    v.extend(chars(ZalgoKind::Down));

    v
}

/// Returns a `Vec` of Zalgo `char`s depending on the value of the given
/// `ZalgoKind`. Each type has its own set of `char`s to be used.
///
/// For retrieval of _all_ `char`s, use `zalgo::all()`.
///
/// # Examples
///
/// ```rust
/// use zalgo::ZalgoKind;
///
/// let chars = zalgo::chars(ZalgoKind::Up);
/// ```
pub fn chars<'a>(kind: ZalgoKind) -> Vec<char> {
    // Can't seem to get these on individual lines without messing up the
    // Unicode. A PR to fix this would be welcome.
    match kind {
        ZalgoKind::Up => vec![
            '̍', '̎', '̄', '̅', '̿', '̑', '̆', '̐', '͒', '͗', '͑', '̇', '̈', '̊',
            '͂', '̓', '̈́', '͊', '͋', '͌', '̃', '̂', '̌', '͐', '̀', '́', '̋', '̏',
            '̒', '̓', '̔', '̽', '̉', 'ͣ', 'ͤ', 'ͥ', 'ͦ', 'ͧ', 'ͨ', 'ͩ', 'ͪ', 'ͫ',
            'ͬ', 'ͭ', 'ͮ', 'ͯ', '̾', '͛', '͆', '̚'
        ],
        ZalgoKind::Middle => vec![
            '̕', '̛', '̀', '́', '͘', '̡', '̢', '̧', '̨', '̴', '̵', '̶', '͏', '͜',
            '͝', '͞', '͟', '͠', '͢', '̸', '̷', '͡', '҉'
        ],
        ZalgoKind::Down => vec![
            '̖', '̗', '̘', '̙', '̜', '̝', '̞', '̟', '̠', '̤', '̥', '̦', '̩', '̪',
            '̫', '̬', '̭', '̮', '̯', '̰', '̱', '̲', '̳', '̹', '̺', '̻', '̼', 'ͅ',
            '͇', '͈', '͉', '͍', '͎', '͓', '͔', '͕', '͖', '͙', '͚', '̣'
        ],
    }
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
/// use zalgo::ZalgoSize;
///
/// let _ = zalgo::gen("test", true, true, true, ZalgoSize::Maxi);
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the middle and lower positions,
/// with a minimum amount of Zalgo:
///
/// ```rust
/// use zalgo::ZalgoSize;
///
/// let _ = zalgo::gen("test", false, true, true, ZalgoSize::Mini);
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the lower position, with a
/// random amount of Zalgo (can be a low amount or high amount):
///
/// ```rust
/// use zalgo::ZalgoSize;
///
/// let _ = zalgo::gen("test", false, false, true, ZalgoSize::None);
/// ```
///
/// Consequentially, you can also not modify your given text with any Zalgo:
///
/// ```rust
/// use zalgo::ZalgoSize;
///
/// let _ = zalgo::gen("test", false, false, false, ZalgoSize::None);
/// // Technically the `ZalgoSize` value given does not matter here.
/// ```
pub fn gen<S: Into<String>>(text: S,
                            up: bool,
                            middle: bool,
                            down: bool,
                            size: ZalgoSize)
                            -> String {
    let val = text.into();

    // The base String where the original text and new Zalgo text will be
    // appended to.
    let mut result = String::new();

    let mut rng = thread_rng();

    let max_up = chars(ZalgoKind::Up).len();
    let max_middle = chars(ZalgoKind::Middle).len();
    let max_down = chars(ZalgoKind::Down).len();

    for ch in val.chars() {
        // Skip the text if it's already a Zalgo char
        if is_zalgo(ch) {
            continue;
        }

        // Push the given character to the resultant string no matter what
        result.push(ch);

        let count_up = rng.gen_range(0, max_up);
        let count_mid = if size == ZalgoSize::None {
            rng.gen_range(0, max_middle) / 2
        } else {
            rng.gen_range(0, max_middle)
        };
        let count_down = rng.gen_range(0, max_down);

        if up {
            for _ in 0..count_up {
                let get = rng.gen_range(0, count_up);
                result.push(*chars(ZalgoKind::Up).get(get).unwrap());
            }
        }

        if middle {
            for _ in 0..count_mid {
                let get = rng.gen_range(0, count_mid);
                result.push(*chars(ZalgoKind::Middle).get(get).unwrap());
            }
        }

        if down {
            for _ in 0..count_down {
                let get = rng.gen_range(0, count_down);
                result.push(*chars(ZalgoKind::Down).get(get).unwrap());
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
