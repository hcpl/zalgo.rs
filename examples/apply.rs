extern crate zalgo;

use zalgo::{CharKind, Intensity};

// Generate a string of Zalgo with a low amount of custom configuration.
fn main() {
    // Create Zalgo text with Zalgo `char`s in all positions, with a maximum
    // amount of Zalgo:
    let _ = zalgo::apply("test", CharKind::all(), Intensity::Maxi);

    // Create Zalgo text with Zalgo `char`s in only the middle and lower
    // positions, with a minimum amount of Zalgo:
    let _ = zalgo::apply("test", CharKind::MIDDLE | CharKind::DOWN, Intensity::Mini);

    // Create Zalgo text with Zalgo `char`s in only the lower position, with a
    // random amount of Zalgo (can be a low amount or high amount):
    let _ = zalgo::apply("test", CharKind::DOWN, Intensity::Random);

    // Consequentially, you can also not modify your given text with any Zalgo:
    // Technically the `Intensity` value given does not matter here.
    let _ = zalgo::apply("test", CharKind::empty(), Intensity::Random);
}
