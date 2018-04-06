extern crate zalgo;

use std::env;
use zalgo::{ZalgoKind, ZalgoSize};

// A bin for generating a binary that can take text as an input and output
// zalgo'd text.
//
// To use this, simply run `cargo run --bin zalgo`.
//
// Pass argument like so:
//
// `cargo run --bin zalgo your text here`.
fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();
    let text: String = args.join(" ");

    let output: String = zalgo::gen(text, ZalgoKind::UP | ZalgoKind::DOWN, ZalgoSize::Mini);

    println!("{}", output);
}
