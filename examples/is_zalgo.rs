extern crate zalgo;

// Test if a given `char` is used for zalgo generation of strings.
fn main() {
    let _: bool = zalgo::is_zalgo( '҉');

    // The following is simply a latin letter, and would return `false` as it is
    // not zalgo.
    let _: bool = zalgo::is_zalgo('a');
}
