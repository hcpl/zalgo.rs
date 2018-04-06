extern crate zalgo;

use zalgo::{ZalgoKind, ZalgoSize};

// Generate a string of Zalgo with a low amount of custom configuration.
fn main() {
    // Create Zalgo text with Zalgo `char`s in all positions, with a maximum
    // amount of Zalgo:
    let _ = zalgo::gen("test", ZalgoKind::all(), ZalgoSize::Maxi);

    // Create Zalgo text with Zalgo `char`s in only the middle and lower
    // positions, with a minimum amount of Zalgo:
    let _ = zalgo::gen("test", ZalgoKind::MIDDLE | ZalgoKind::DOWN, ZalgoSize::Mini);

    // Create Zalgo text with Zalgo `char`s in only the lower position, with a
    // random amount of Zalgo (can be a low amount or high amount):
    let _ = zalgo::gen("test", ZalgoKind::DOWN, ZalgoSize::None);

    // Consequentially, you can also not modify your given text with any Zalgo:
    // Technically the `ZalgoSize` value given does not matter here.
    let _ = zalgo::gen("test", ZalgoKind::empty(), ZalgoSize::None);
}
