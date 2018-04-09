extern crate zalgo;

// Retrieve an iterator of `chars` of all zalgo characters.
fn main() {
    for (i, ch) in zalgo::all_chars().enumerate() {
        println!("Zalgo char #{}: `{:?}` (codepoint {})", i + 1, ch, ch as u32);
    }
}
