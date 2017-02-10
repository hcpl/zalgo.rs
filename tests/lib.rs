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
fn enums() {
    let _ = ZalgoKind::Up;
    let _ = ZalgoKind::Middle;
    let _ = ZalgoKind::Down;

    let _ = ZalgoSize::Maxi;
    let _ = ZalgoSize::Mini;
    let _ = ZalgoSize::None;
}

#[test]
fn gen() {
    // It's not really possible to test the outputs, so just test whether they
    // work or not.
    let _ = zalgo::gen("t", true, true, true, ZalgoSize::Mini);
    let _ = zalgo::gen("t", true, true, false, ZalgoSize::Mini);
    let _ = zalgo::gen("t", true, false, true, ZalgoSize::Mini);
    let _ = zalgo::gen("t", true, false, false, ZalgoSize::Mini);
    let _ = zalgo::gen("t", false, false, false, ZalgoSize::Mini);
    let _ = zalgo::gen("t", false, false, true, ZalgoSize::Mini);
    let _ = zalgo::gen("t", false, true, false, ZalgoSize::Mini);
    let _ = zalgo::gen("t", false, true, true, ZalgoSize::Mini);

    let _ = zalgo::gen("t", true, true, true, ZalgoSize::Maxi);
    let _ = zalgo::gen("t", true, true, false, ZalgoSize::Maxi);
    let _ = zalgo::gen("t", true, false, true, ZalgoSize::Maxi);
    let _ = zalgo::gen("t", true, false, false, ZalgoSize::Maxi);
    let _ = zalgo::gen("t", false, false, false, ZalgoSize::Maxi);
    let _ = zalgo::gen("t", false, false, true, ZalgoSize::Maxi);
    let _ = zalgo::gen("t", false, true, false, ZalgoSize::Maxi);
    let _ = zalgo::gen("t", false, true, true, ZalgoSize::Maxi);

    let _ = zalgo::gen("t", true, true, true, ZalgoSize::None);
    let _ = zalgo::gen("t", true, true, false, ZalgoSize::None);
    let _ = zalgo::gen("t", true, false, true, ZalgoSize::None);
    let _ = zalgo::gen("t", true, false, false, ZalgoSize::None);
    let _ = zalgo::gen("t", false, false, false, ZalgoSize::None);
    let _ = zalgo::gen("t", false, false, true, ZalgoSize::None);
    let _ = zalgo::gen("t", false, true, false, ZalgoSize::None);
    let _ = zalgo::gen("t", false, true, true, ZalgoSize::None);

    // Test that passing a String works.
    let _ = zalgo::gen(String::from("t"), false, false, false, ZalgoSize::None);
}
