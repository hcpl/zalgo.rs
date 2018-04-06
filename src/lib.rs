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
//! use zalgo::{CharKind, Intensity};
//!
//! let result = zalgo::gen("my string", CharKind::UP, Intensity::Maxi);
//! ```

#[macro_use]
extern crate bitflags;
extern crate rand;

use rand::{thread_rng, Rng};

pub static DESCRIPTION: &str = "\
    To invoke the hive-mind representing chaos.\n\
    Invoking the feeling of chaos.\n\
    With out order.\n\
    The Nezperdian hive-mind of chaos. Zalgo.\n\
    He who Waits Behind The Wall.\n\
    ZALGO!\
";

/// Denotes characters to be used in the "upper" part of text.
pub static UP_CHARS: [char; 50] = [
    '̍', '̎', '̄', '̅', '̿', '̑', '̆', '̐', '͒', '͗', '͑', '̇', '̈', '̊',
    '͂', '̓', '̈́', '͊', '͋', '͌', '̃', '̂', '̌', '͐', '̀', '́', '̋', '̏',
    '̒', '̓', '̔', '̽', '̉', 'ͣ', 'ͤ', 'ͥ', 'ͦ', 'ͧ', 'ͨ', 'ͩ', 'ͪ', 'ͫ',
    'ͬ', 'ͭ', 'ͮ', 'ͯ', '̾', '͛', '͆', '̚',
];
/// Denotes characters to be used in the "middle" part of text.
pub static MIDDLE_CHARS: [char; 23] = [
    '̕', '̛', '̀', '́', '͘', '̡', '̢', '̧', '̨', '̴', '̵', '̶', '͏', '͜',
    '͝', '͞', '͟', '͠', '͢', '̸', '̷', '͡', '҉',
];
/// Denotes characters to be used in the "bottom" part of text.
pub static DOWN_CHARS: [char; 40] = [
    '̖', '̗', '̘', '̙', '̜', '̝', '̞', '̟', '̠', '̤', '̥', '̦', '̩', '̪',
    '̫', '̬', '̭', '̮', '̯', '̰', '̱', '̲', '̳', '̹', '̺', '̻', '̼', 'ͅ',
    '͇', '͈', '͉', '͍', '͎', '͓', '͔', '͕', '͖', '͙', '͚', '̣',
];

bitflags! {
    /// A definition of the character type to be used for retrieval.
    pub struct CharKind: u8 {
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

/// The intensity of the Zalgo text within the string to produce.
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Intensity {
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

    v.extend(UP_CHARS.iter());
    v.extend(MIDDLE_CHARS.iter());
    v.extend(DOWN_CHARS.iter());

    v
}

/// Generates a String containing Zalgo text. This is customizable via defining
/// whether to include Zalgo text above the given string, in the middle of it,
/// and below it.
///
/// The amount of Zalgo text can be (more or less) defined by the value of the
/// `intensity` given. Read on the `Intensity` for more information.
///
/// # Notes on random generator
///
/// This function uses [`rand::thread_rng`] under the hood which contiributes to
/// its non-determinism. For reproducible results (such as when performing
/// tests) or when other random generator is needed use [`gen_rng`] instead and
/// provide it a random generator of your choice.
///
/// # Examples
///
/// Create Zalgo text with Zalgo `char`s in all positions, with a maximum amount
/// of Zalgo:
///
/// ```rust
/// use zalgo::{CharKind, Intensity};
///
/// let _ = zalgo::gen("test", CharKind::all(), Intensity::Maxi);
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the middle and lower positions,
/// with a minimum amount of Zalgo:
///
/// ```rust
/// use zalgo::{CharKind, Intensity};
///
/// let _ = zalgo::gen("test", CharKind::MIDDLE | CharKind::DOWN, Intensity::Mini);
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the lower position, with a
/// random amount of Zalgo (can be a low amount or high amount):
///
/// ```rust
/// use zalgo::{CharKind, Intensity};
///
/// let _ = zalgo::gen("test", CharKind::DOWN, Intensity::None);
/// ```
///
/// Consequentially, you can also not modify your given text with any Zalgo:
///
/// ```rust
/// use zalgo::{CharKind, Intensity};
///
/// let _ = zalgo::gen("test", CharKind::empty(), Intensity::None);
/// // Technically the `Intensity` value given does not matter here.
/// ```
///
/// [`rand::thread_rng`]: https://docs.rs/rand/^0.4/rand/fn.thread_rng.html
/// [`gen_rng`]: fn.gen_rng.html
pub fn gen<S: Into<String>>(text: S, kind: CharKind, intensity: Intensity) -> String {
    gen_rng(&mut thread_rng(), text, kind, intensity)
}

/// Version of [`gen`] function generic over [`rand::Rng`].
///
/// # Examples
///
/// Create Zalgo text with Zalgo `char`s in all positions, with a maximum amount
/// of Zalgo:
///
/// ```rust
/// # extern crate rand;
/// # extern crate zalgo;
/// use rand::IsaacRng;
/// use zalgo::{CharKind, Intensity};
///
/// # fn main() {
/// let mut rng = IsaacRng::new_unseeded();
/// let s = zalgo::gen_rng(&mut rng, "test", CharKind::all(), Intensity::Maxi);
///
/// assert_eq!(s, "t̡̢̢̡̡̢̂͊̿̌̊̄̑̅͗͒̂̆̐̎̂͒̍̄͊͑̿̈͌͘̕͏̢̧̛̹̘̪̫̝̞̝͉̤̱̝̠̼̘̼͉̤̝̗͎̳̮̰̤͇͎͍̝̥̜̮̖̀́ͅͅę̵̶̵̷̶̶̢̛̜̞̠̜̜̗̝̞̤̤̖͂͗̈̓͗̿͂͋͊̅͒̇̆̑̎͂̿͑͒̀́́́́̕͘͢͟͢͠͝͞s̡̰̮̙̱̳̼̹̭̗̪̻̖̱̬̲̥̘̝̹̲̫̱̞̪̳̳̺̎͗͒͂̑̈͂̐̊̐̇̓̇̓̍̐̅̎͗̀̀̀́̕ţ̴̡̧̧̄̅̅̐̍̐̄̅́̕̕͞͝͏̢̤̟̤̜̟̜̤̘̖̜̗̜̙̀̀͜͝͞͞");
/// # }
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the middle and lower positions,
/// with a minimum amount of Zalgo:
///
/// ```rust
/// # extern crate rand;
/// # extern crate zalgo;
/// use rand::IsaacRng;
/// use zalgo::{CharKind, Intensity};
///
/// # fn main() {
/// let mut rng = IsaacRng::new_unseeded();
/// let s = zalgo::gen_rng(&mut rng, "test", CharKind::MIDDLE | CharKind::DOWN, Intensity::Mini);
///
/// assert_eq!(s, "t̵̨̢̛̛̛̛͈̼̜̗̙̮̖̖͉̲̯̳̻̞̗̯̘̘̯̥̪̙͓̹̘̪̫̝̞̝͉̤̱́́̕͘͘̕͜ͅe̘̫̭̤̰̟̞̜̟̮̲̥̘̙̬̯̖̙̮̯̪̘ś̡̢̛̛̗̘̗̀̀͘t̢̧̢̘̗̲̩̦̜̰̮̙̫̹̙̟̹̲̱̖̜̪̪̹̩̟̰̫́́̀̕͘̕");
/// # }
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the lower position, with a
/// random amount of Zalgo (can be a low amount or high amount):
///
/// ```rust
/// # extern crate rand;
/// # extern crate zalgo;
/// use rand::IsaacRng;
/// use zalgo::{CharKind, Intensity};
///
/// # fn main() {
/// let mut rng = IsaacRng::new_unseeded();
/// let s = zalgo::gen_rng(&mut rng, "test", CharKind::DOWN, Intensity::None);
///
/// assert_eq!(s, "t̤̗̯̠̗̜̙̹̥͎̩̹̹̺͈̼̜̗̙̮̖̖͉̲̯̳̻̞̗̯̘̘̯̥e͕̩͖̮̻̗͈̤̳̫̬̝̼͇̞͈͉͇̙͔͔̫̯͓̬͖̥̹̟̬̲̻̦̥͈̭͉s̟̟̠̜̗̠̝̗̘t̫͉̤̙̥̰̺̖̦̙̻̮̻͈̥̤̝̯̦̻̼̜͇̦̗̻̜̮̠̼̜̩ͅ");
/// # }
/// ```
///
/// Consequentially, you can also not modify your given text with any Zalgo:
///
/// ```rust
/// # extern crate rand;
/// # extern crate zalgo;
/// use rand::IsaacRng;
/// use zalgo::{CharKind, Intensity};
///
/// # fn main() {
/// let mut rng = IsaacRng::new_unseeded();
/// let s = zalgo::gen_rng(&mut rng, "test", CharKind::empty(), Intensity::None);
/// // Technically the `Intensity` value given does not matter here.
///
/// assert_eq!(s, "test");
/// # }
/// ```
///
/// [`gen`]: fn.gen.html
/// [`rand::Rng`]: https://docs.rs/rand/^0.4/rand/trait.Rng.html
pub fn gen_rng<S: Into<String>, R: Rng>(
    rng: &mut R,
    text: S,
    kind: CharKind,
    intensity: Intensity,
) -> String {
    let val = text.into();

    // The base String where the original text and new Zalgo text will be
    // appended to.
    let mut result = String::new();

    for ch in val.chars() {
        // Skip the text if it's already a Zalgo char
        if is_zalgo(ch) {
            continue;
        }

        // Push the given character to the resultant string no matter what
        result.push(ch);

        let count_up = rng.gen_range(0, UP_CHARS.len());
        let count_mid = if intensity == Intensity::None {
            rng.gen_range(0, MIDDLE_CHARS.len()) / 2
        } else {
            rng.gen_range(0, MIDDLE_CHARS.len())
        };
        let count_down = rng.gen_range(0, DOWN_CHARS.len());

        if kind.contains(CharKind::UP) {
            for _ in 0..count_up {
                let get = rng.gen_range(0, count_up);
                result.push(UP_CHARS[get]);
            }
        }

        if kind.contains(CharKind::MIDDLE) {
            for _ in 0..count_mid {
                let get = rng.gen_range(0, count_mid);
                result.push(MIDDLE_CHARS[get]);
            }
        }

        if kind.contains(CharKind::DOWN) {
            for _ in 0..count_down {
                let get = rng.gen_range(0, count_down);
                result.push(DOWN_CHARS[get]);
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
