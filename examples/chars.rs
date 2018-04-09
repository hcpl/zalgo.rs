extern crate zalgo;

// Retrieve an `Iterator` of `char`s for a specific position of zalgo text, e.g.
// the 'top' characters or 'middle' characters.
fn main() {
    // Retrieve all characters used for the 'top' of the resultant string.
    println!("Up chars: {:?}", zalgo::UP_CHARS.as_ref());

    // Retrieve all characters used for the 'middle' of the resultant string.
    println!("Middle chars: {:?}", zalgo::MIDDLE_CHARS.as_ref());

    // Retrieve all characters used for the 'down' of the resultant string.
    println!("Down chars: {:?}", zalgo::DOWN_CHARS.as_ref());
}
