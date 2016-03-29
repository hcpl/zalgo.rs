extern crate zalgo;

use zalgo::ZalgoKind;

// Retrieve a `Vec` of `char`s for a specific position of zalgo text, e.g.
// the 'top' characters or 'middle' characters.
fn main() {
    // Retrieve all characters used for the 'top' of the resultant string.
    let _ = zalgo::chars(ZalgoKind::Up);

    // Retrieve all characters used for the 'middle' of the resultant string.
    let _ = zalgo::chars(ZalgoKind::Middle);
}
