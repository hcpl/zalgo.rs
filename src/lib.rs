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
//! # #[cfg(feature = "std")]
//! let result = zalgo::apply("my string", CharKind::UP, Intensity::Maxi);
//! ```
//!
//! ## Optional features
//!
//! - **`std`** *(enabled by default)* — `libstd` support. If disabled, `zalgo`
//!   will use `libcore` instead.
//! - **`alloc`** — `liballoc` support which provides heap-allocated facilities
//!   (currently requires nightly rustc). Enable this to use functions that
//!   operate on `String`s and don't depend on std-only capabilities like
//!   thread-local RNGs when `std` is disabled.
//! - **`nightly`** — Access to unstable features available on nightly
//!   compilers.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]
#![cfg_attr(feature = "nightly", feature(
    exact_size_is_empty,
    fused,
    iter_rfold,
    iterator_try_fold,
    trusted_len,
    try_trait,
))]

#[cfg(feature = "std")]
extern crate std as core;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[macro_use]
extern crate bitflags;
extern crate rand;

mod all_chars;
pub use all_chars::{AllChars, all_chars};

mod apply_rng_iter;
pub use apply_rng_iter::{ApplyRngIter, apply_rng_iter};

mod unapply_iter;
pub use unapply_iter::{UnapplyIter, unapply_iter};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::String;

#[cfg(any(feature = "std", feature = "alloc"))]
use rand::Rng;
#[cfg(feature = "std")]
use rand::{ThreadRng, thread_rng};


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
    /// Produce a smaller amount of Zalgo text.
    Mini,
    /// Produce a normal amount of Zalgo text.
    Normal,
    /// Produce a larger amount of Zalgo text.
    Maxi,
    /// Produce a randomized amount of Zalgo text.
    Random,
    /// Produce a customized amount of Zalgo text.
    Custom {
        up: usize,
        middle: usize,
        down: usize,
    },
}

/// Generates a `String` containing Zalgo text with thread-local random
/// generator.
///
/// The output is customizable via defining whether to include Zalgo text above
/// the given string, in the middle of it, and below it.
///
/// The amount of Zalgo text can be (more or less) defined by the value of the
/// `intensity` given. Read on the [`Intensity`] for more information.
///
/// *This function is available if Zalgo is built with the `"std"` feature.*
///
/// # Notes on random generator
///
/// This function uses [`rand::thread_rng`] under the hood which contiributes to
/// its non-determinism. For reproducible results (such as when performing
/// tests) or when other random generator is needed use [`apply_rng`] instead and
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
/// let _ = zalgo::apply("test", CharKind::all(), Intensity::Maxi);
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the middle and lower positions,
/// with a minimum amount of Zalgo:
///
/// ```rust
/// use zalgo::{CharKind, Intensity};
///
/// let _ = zalgo::apply("test", CharKind::MIDDLE | CharKind::DOWN, Intensity::Mini);
/// ```
///
/// Create Zalgo text with Zalgo `char`s in only the lower position, with a
/// random amount of Zalgo (can be a low amount or high amount):
///
/// ```rust
/// use zalgo::{CharKind, Intensity};
///
/// let _ = zalgo::apply("test", CharKind::DOWN, Intensity::Random);
/// ```
///
/// Consequentially, you can also not modify your given text with any Zalgo:
///
/// ```rust
/// use zalgo::{CharKind, Intensity};
///
/// let _ = zalgo::apply("test", CharKind::empty(), Intensity::Random);
/// // Technically the `Intensity` value given does not matter here.
/// ```
///
/// [`Intensity`]: enum.Intensity.html
/// [`rand::thread_rng`]: https://docs.rs/rand/^0.4/rand/fn.thread_rng.html
/// [`apply_rng`]: fn.apply_rng.html
#[cfg(feature = "std")]
pub fn apply(text: &str, kind: CharKind, intensity: Intensity) -> String {
    apply_rng_iter(thread_rng(), text.chars(), kind, intensity).collect()
}

/// Generates a `String` containing Zalgo text with user-provided random
/// generator.
///
/// The output is customizable via defining whether to include Zalgo text above
/// the given string, in the middle of it, and below it.
///
/// The amount of Zalgo text can be (more or less) defined by the value of the
/// `intensity` given. Read on the [`Intensity`] for more information.
///
/// *This function is available if Zalgo is built with the `"std"` or `"alloc"`
/// feature.*
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
/// let s = zalgo::apply_rng(&mut rng, "test", CharKind::all(), Intensity::Maxi);
///
/// assert_eq!(s, "ţ̛̭̼̹͕̑͗̾͆͛̿̐ͯ̈͆ͦ̊͡ͅeͧ̌͂̐͋ͫ̃̈́̆ͦ͒̑ͩ͛ͩ͋ͧ͒͘͟͏̭͓̥̣͚̳̞̯̩͈͍̝͖̠͞s̶̜̲̗̞͓̻̰̘̯͔͇̭̰͒̾ͣͧͦ͒̊̏̃͌͒ͫ̂̊͗̓̓̈́͢͢ͅt̷̳̪͈̙̯̩͓͍̎̾̈ͦ̿");
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
/// let s = zalgo::apply_rng(&mut rng, "test", CharKind::MIDDLE | CharKind::DOWN, Intensity::Mini);
///
/// assert_eq!(s, "t̝̤e̫̮̝͎͚̭̪s̭̼̹t͏͚̳̜͙͚ͅ");
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
/// let s = zalgo::apply_rng(&mut rng, "test", CharKind::DOWN, Intensity::Random);
///
/// assert_eq!(s, "t̤̞̠̟̫̮̝e̗̳̰̭̼s̜͚̳̜͙͚͍ͅt");
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
/// let s = zalgo::apply_rng(&mut rng, "test", CharKind::empty(), Intensity::Random);
/// // Technically the `Intensity` value given does not matter here.
///
/// assert_eq!(s, "test");
/// # }
/// ```
///
/// [`Intensity`]: enum.Intensity.html
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn apply_rng<R: Rng>(
    rng: R,
    text: &str,
    kind: CharKind,
    intensity: Intensity,
) -> String {
    apply_rng_iter(rng, text.chars(), kind, intensity).collect()
}

/// Returns an [`Iterator`] of `char`s of generated Zalgo text with thread-local
/// random generator.
///
/// The output is customizable via defining whether to include Zalgo text above
/// the given string, in the middle of it, and below it.
///
/// The amount of Zalgo text can be (more or less) defined by the value of the
/// `intensity` given. Read on the [`Intensity`] for more information.
///
/// *This function is available if Zalgo is built with the `"std"` feature.*
///
/// # Notes on random generator
///
/// This function uses [`rand::thread_rng`] under the hood which contiributes to
/// its non-determinism. For reproducible results (such as when performing
/// tests) or when other random generator is needed use [`apply_rng_ter`]
/// instead and provide it a random generator of your choice.
///
/// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
/// [`Intensity`]: enum.Intensity.html
/// [`rand::thread_rng`]: https://docs.rs/rand/^0.4/rand/fn.thread_rng.html
/// [`apply_rng_iter`]: fn.apply_rng_iter.html
#[cfg(feature = "std")]
pub fn apply_iter<I: Iterator<Item = char>>(
    chars: I,
    kind: CharKind,
    intensity: Intensity
) -> ApplyRngIter<ThreadRng, I> {
    apply_rng_iter(thread_rng(), chars, kind, intensity)
}

/// Removes Zalgo `char`s from the original string.
///
/// *This function is available if Zalgo is built with the `"std"` or `"alloc"`
/// feature.*
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn unapply(text: &str) -> String {
    unapply_iter(text.chars()).collect()
}

/// Determines whether a given `char` is a Zalgo `char`. This is checked by
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
    all_chars().find(|c| *c == ch).is_some()
}
