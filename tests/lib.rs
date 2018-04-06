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

#![cfg_attr(feature = "nightly", feature(exact_size_is_empty))]

extern crate zalgo;

use zalgo::{UP_CHARS, MIDDLE_CHARS, DOWN_CHARS, CharKind, Intensity};

#[test]
fn all_chars() {
    let all_chars_count = 113;

    assert_eq!(UP_CHARS.len() + MIDDLE_CHARS.len() + DOWN_CHARS.len(), all_chars_count);
    assert_eq!(zalgo::all_chars().len(), all_chars_count);
    assert_eq!(zalgo::all_chars().count(), all_chars_count);
    assert_eq!(zalgo::all_chars().size_hint(), (all_chars_count, Some(all_chars_count)));
    assert_eq!(zalgo::all_chars().last(), DOWN_CHARS.last().cloned());

    let mut all_chars = zalgo::all_chars();

    for i in 0..all_chars_count {
        let remaining = all_chars_count - i;
        assert_eq!(all_chars.len(), remaining);
        assert_eq!(all_chars.size_hint(), (remaining, Some(remaining)));
        assert!(all_chars.len() > 0);
        #[cfg(feature = "nightly")]
        assert!(!all_chars.is_empty());

        let c = all_chars.next();
        assert!(c.is_some());

        let actual_c = if i < UP_CHARS.len() {
            UP_CHARS[i]
        } else if i - UP_CHARS.len() < MIDDLE_CHARS.len() {
            MIDDLE_CHARS[i - UP_CHARS.len()]
        } else if i - UP_CHARS.len() - MIDDLE_CHARS.len() < DOWN_CHARS.len() {
            DOWN_CHARS[i - UP_CHARS.len() - MIDDLE_CHARS.len()]
        } else {
            panic!("Shouldn't happen");
        };

        assert_eq!(c, Some(actual_c));

        assert_eq!(all_chars.len(), remaining - 1);
        assert_eq!(all_chars.size_hint(), (remaining - 1, Some(remaining - 1)));
    }

    assert_eq!(all_chars.next(), None);
    assert_eq!(all_chars.next_back(), None);
    assert_eq!(all_chars.len(), 0);
    #[cfg(feature = "nightly")]
    assert!(all_chars.is_empty());

    let mut all_chars = zalgo::all_chars();

    for i in (0..all_chars_count).rev() {
        let remaining = i + 1;
        assert_eq!(all_chars.len(), remaining);
        assert_eq!(all_chars.size_hint(), (remaining, Some(remaining)));
        assert!(all_chars.len() > 0);
        #[cfg(feature = "nightly")]
        assert!(!all_chars.is_empty());

        let c = all_chars.next_back();
        assert!(c.is_some());

        let actual_c = if i < UP_CHARS.len() {
            UP_CHARS[i]
        } else if i - UP_CHARS.len() < MIDDLE_CHARS.len() {
            MIDDLE_CHARS[i - UP_CHARS.len()]
        } else if i - UP_CHARS.len() - MIDDLE_CHARS.len() < DOWN_CHARS.len() {
            DOWN_CHARS[i - UP_CHARS.len() - MIDDLE_CHARS.len()]
        } else {
            panic!("Shouldn't happen");
        };

        assert_eq!(c, Some(actual_c));

        assert_eq!(all_chars.len(), remaining - 1);
        assert_eq!(all_chars.size_hint(), (remaining - 1, Some(remaining - 1)));
    }

    assert_eq!(all_chars.next(), None);
    assert_eq!(all_chars.next_back(), None);
    assert_eq!(all_chars.len(), 0);
    #[cfg(feature = "nightly")]
    assert!(all_chars.is_empty());
}

#[test]
fn char_kind() {
    let empty = CharKind::empty();

    let up = CharKind::UP;
    let middle = CharKind::MIDDLE;
    let down = CharKind::DOWN;

    let all = CharKind::all();

    assert_eq!(empty, up & middle);
    assert_eq!(empty, up & down);
    assert_eq!(empty, middle & down);
    assert_eq!(empty, up & middle & down);
    assert!(empty.contains(empty));
    assert!(!empty.contains(up));
    assert!(!empty.contains(middle));
    assert!(!empty.contains(down));
    assert!(!empty.contains(up | middle));
    assert!(!empty.contains(up | down));
    assert!(!empty.contains(middle | down));
    assert!(!empty.contains(up | middle | down));

    assert_eq!(all, up | middle | down);
    assert!(all.contains(empty));
    assert!(all.contains(up));
    assert!(all.contains(middle));
    assert!(all.contains(down));
    assert!(all.contains(up | middle));
    assert!(all.contains(up | down));
    assert!(all.contains(middle | down));
    assert!(all.contains(up | middle | down));

    assert_eq!(up, (up | middle) & (up | down));
    assert_eq!(middle, (up | middle) & (middle | down));
    assert_eq!(down, (up | down) & (middle | down));

    assert_eq!(!up, middle | down);
    assert_eq!(!middle, up | down);
    assert_eq!(!down, up | middle);

    assert_eq!(CharKind::from_bits(0b000), Some(empty));
    assert_eq!(CharKind::from_bits(0b001), Some(up));
    assert_eq!(CharKind::from_bits(0b010), Some(middle));
    assert_eq!(CharKind::from_bits(0b100), Some(down));
    assert_eq!(CharKind::from_bits(0b011), Some(up | middle));
    assert_eq!(CharKind::from_bits(0b101), Some(up | down));
    assert_eq!(CharKind::from_bits(0b110), Some(middle | down));
    assert_eq!(CharKind::from_bits(0b111), Some(all));

    assert_eq!(CharKind::from_bits(0b1011), None);
    assert_eq!(CharKind::from_bits(0b00100000), None);
}

#[test]
fn intensity() {
    let _ = Intensity::Mini;
    let _ = Intensity::Normal;
    let _ = Intensity::Maxi;
    let _ = Intensity::Random;
}

#[test]
fn apply() {
    // It's not really possible to test the outputs, so just test whether they
    // work or not.
    let _ = zalgo::apply("t", CharKind::from_bits(0b000).unwrap(), Intensity::Mini);
    let _ = zalgo::apply("t", CharKind::from_bits(0b001).unwrap(), Intensity::Mini);
    let _ = zalgo::apply("t", CharKind::from_bits(0b010).unwrap(), Intensity::Mini);
    let _ = zalgo::apply("t", CharKind::from_bits(0b100).unwrap(), Intensity::Mini);
    let _ = zalgo::apply("t", CharKind::from_bits(0b011).unwrap(), Intensity::Mini);
    let _ = zalgo::apply("t", CharKind::from_bits(0b101).unwrap(), Intensity::Mini);
    let _ = zalgo::apply("t", CharKind::from_bits(0b110).unwrap(), Intensity::Mini);
    let _ = zalgo::apply("t", CharKind::from_bits(0b111).unwrap(), Intensity::Mini);

    let _ = zalgo::apply("t", CharKind::from_bits(0b000).unwrap(), Intensity::Normal);
    let _ = zalgo::apply("t", CharKind::from_bits(0b001).unwrap(), Intensity::Normal);
    let _ = zalgo::apply("t", CharKind::from_bits(0b010).unwrap(), Intensity::Normal);
    let _ = zalgo::apply("t", CharKind::from_bits(0b100).unwrap(), Intensity::Normal);
    let _ = zalgo::apply("t", CharKind::from_bits(0b011).unwrap(), Intensity::Normal);
    let _ = zalgo::apply("t", CharKind::from_bits(0b101).unwrap(), Intensity::Normal);
    let _ = zalgo::apply("t", CharKind::from_bits(0b110).unwrap(), Intensity::Normal);
    let _ = zalgo::apply("t", CharKind::from_bits(0b111).unwrap(), Intensity::Normal);

    let _ = zalgo::apply("t", CharKind::from_bits(0b000).unwrap(), Intensity::Maxi);
    let _ = zalgo::apply("t", CharKind::from_bits(0b001).unwrap(), Intensity::Maxi);
    let _ = zalgo::apply("t", CharKind::from_bits(0b010).unwrap(), Intensity::Maxi);
    let _ = zalgo::apply("t", CharKind::from_bits(0b100).unwrap(), Intensity::Maxi);
    let _ = zalgo::apply("t", CharKind::from_bits(0b011).unwrap(), Intensity::Maxi);
    let _ = zalgo::apply("t", CharKind::from_bits(0b101).unwrap(), Intensity::Maxi);
    let _ = zalgo::apply("t", CharKind::from_bits(0b110).unwrap(), Intensity::Maxi);
    let _ = zalgo::apply("t", CharKind::from_bits(0b111).unwrap(), Intensity::Maxi);

    let _ = zalgo::apply("t", CharKind::from_bits(0b000).unwrap(), Intensity::Random);
    let _ = zalgo::apply("t", CharKind::from_bits(0b001).unwrap(), Intensity::Random);
    let _ = zalgo::apply("t", CharKind::from_bits(0b010).unwrap(), Intensity::Random);
    let _ = zalgo::apply("t", CharKind::from_bits(0b100).unwrap(), Intensity::Random);
    let _ = zalgo::apply("t", CharKind::from_bits(0b011).unwrap(), Intensity::Random);
    let _ = zalgo::apply("t", CharKind::from_bits(0b101).unwrap(), Intensity::Random);
    let _ = zalgo::apply("t", CharKind::from_bits(0b110).unwrap(), Intensity::Random);
    let _ = zalgo::apply("t", CharKind::from_bits(0b111).unwrap(), Intensity::Random);

    // Test that passing a String works.
    let _ = zalgo::apply(&String::from("t"), CharKind::empty(), Intensity::Random);
}
