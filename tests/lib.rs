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

extern crate zalgo;

use zalgo::{ZalgoKind, ZalgoSize};

#[test]
fn all() {
    assert!(zalgo::all().len() == 113);
}

#[test]
fn bitflags() {
    let empty = ZalgoKind::empty();

    let up = ZalgoKind::UP;
    let middle = ZalgoKind::MIDDLE;
    let down = ZalgoKind::DOWN;

    let all = ZalgoKind::all();

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

    assert_eq!(ZalgoKind::from_bits(0b000), Some(empty));
    assert_eq!(ZalgoKind::from_bits(0b001), Some(up));
    assert_eq!(ZalgoKind::from_bits(0b010), Some(middle));
    assert_eq!(ZalgoKind::from_bits(0b100), Some(down));
    assert_eq!(ZalgoKind::from_bits(0b011), Some(up | middle));
    assert_eq!(ZalgoKind::from_bits(0b101), Some(up | down));
    assert_eq!(ZalgoKind::from_bits(0b110), Some(middle | down));
    assert_eq!(ZalgoKind::from_bits(0b111), Some(all));

    assert_eq!(ZalgoKind::from_bits(0b1011), None);
    assert_eq!(ZalgoKind::from_bits(0b00100000), None);
}

#[test]
fn enums() {
    let _ = ZalgoSize::Maxi;
    let _ = ZalgoSize::Mini;
    let _ = ZalgoSize::None;
}

#[test]
fn gen() {
    // It's not really possible to test the outputs, so just test whether they
    // work or not.
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b000).unwrap(), ZalgoSize::Mini);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b001).unwrap(), ZalgoSize::Mini);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b010).unwrap(), ZalgoSize::Mini);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b100).unwrap(), ZalgoSize::Mini);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b011).unwrap(), ZalgoSize::Mini);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b101).unwrap(), ZalgoSize::Mini);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b110).unwrap(), ZalgoSize::Mini);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b111).unwrap(), ZalgoSize::Mini);

    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b000).unwrap(), ZalgoSize::Maxi);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b001).unwrap(), ZalgoSize::Maxi);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b010).unwrap(), ZalgoSize::Maxi);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b100).unwrap(), ZalgoSize::Maxi);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b011).unwrap(), ZalgoSize::Maxi);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b101).unwrap(), ZalgoSize::Maxi);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b110).unwrap(), ZalgoSize::Maxi);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b111).unwrap(), ZalgoSize::Maxi);

    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b000).unwrap(), ZalgoSize::None);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b001).unwrap(), ZalgoSize::None);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b010).unwrap(), ZalgoSize::None);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b100).unwrap(), ZalgoSize::None);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b011).unwrap(), ZalgoSize::None);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b101).unwrap(), ZalgoSize::None);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b110).unwrap(), ZalgoSize::None);
    let _ = zalgo::gen("t", ZalgoKind::from_bits(0b111).unwrap(), ZalgoSize::None);

    // Test that passing a String works.
    let _ = zalgo::gen(String::from("t"), ZalgoKind::empty(), ZalgoSize::None);
}
