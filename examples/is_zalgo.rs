extern crate zalgo;

// Test if a given `char` is used for zalgo generation of strings.
fn main() {
    println!("{}", zalgo::is_zalgo( '҉'));

    // The following is simply a latin letter, and would return `false` as it is
    // not zalgo.
    println!("{}", zalgo::is_zalgo('a'));

    // Every char from `zalgo::all_chars()` is a Zalgo char by definition.
    println!("{}", zalgo::all_chars().all(zalgo::is_zalgo));
}
