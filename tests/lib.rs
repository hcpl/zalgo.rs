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

#![cfg_attr(feature = "nightly", feature(exact_size_is_empty))]

extern crate lipsum;
extern crate rand;
extern crate zalgo;

use zalgo::{UP_CHARS, MIDDLE_CHARS, DOWN_CHARS, CharKind, Intensity};


const ALL_CHARS_COUNT: usize = 113;

#[test]
fn all_chars_common_properties() {
    assert_eq!(UP_CHARS.len() + MIDDLE_CHARS.len() + DOWN_CHARS.len(), ALL_CHARS_COUNT);
    assert_eq!(zalgo::all_chars().len(), ALL_CHARS_COUNT);
    assert_eq!(zalgo::all_chars().count(), ALL_CHARS_COUNT);
    assert_eq!(zalgo::all_chars().size_hint(), (ALL_CHARS_COUNT, Some(ALL_CHARS_COUNT)));
    assert_eq!(zalgo::all_chars().last(), DOWN_CHARS.last().cloned());
}

#[test]
fn all_chars_forwards_iteration() {
    let mut all_chars = zalgo::all_chars();

    for i in 0..ALL_CHARS_COUNT {
        let remaining = ALL_CHARS_COUNT - i;
        assert_eq!(all_chars.len(), remaining);
        assert_eq!(all_chars.size_hint(), (remaining, Some(remaining)));
        assert!(all_chars.len() > 0);
        #[cfg(feature = "nightly")]
        assert!(!all_chars.is_empty());

        let c = all_chars.next();
        assert!(c.is_some());

        let actual_c = if i < UP_CHARS.len() {
            UP_CHARS[i]
        } else if i - UP_CHARS.len() < MIDDLE_CHARS.len() {
            MIDDLE_CHARS[i - UP_CHARS.len()]
        } else if i - UP_CHARS.len() - MIDDLE_CHARS.len() < DOWN_CHARS.len() {
            DOWN_CHARS[i - UP_CHARS.len() - MIDDLE_CHARS.len()]
        } else {
            panic!("Shouldn't happen");
        };

        assert_eq!(c, Some(actual_c));

        assert_eq!(all_chars.len(), remaining - 1);
        assert_eq!(all_chars.size_hint(), (remaining - 1, Some(remaining - 1)));
    }

    assert_eq!(all_chars.next(), None);
    assert_eq!(all_chars.next_back(), None);
    assert_eq!(all_chars.len(), 0);
    #[cfg(feature = "nightly")]
    assert!(all_chars.is_empty());
}

#[test]
fn all_chars_backwards_iteration() {
    let mut all_chars = zalgo::all_chars();

    for i in (0..ALL_CHARS_COUNT).rev() {
        let remaining = i + 1;
        assert_eq!(all_chars.len(), remaining);
        assert_eq!(all_chars.size_hint(), (remaining, Some(remaining)));
        assert!(all_chars.len() > 0);
        #[cfg(feature = "nightly")]
        assert!(!all_chars.is_empty());

        let c = all_chars.next_back();
        assert!(c.is_some());

        let actual_c = if i < UP_CHARS.len() {
            UP_CHARS[i]
        } else if i - UP_CHARS.len() < MIDDLE_CHARS.len() {
            MIDDLE_CHARS[i - UP_CHARS.len()]
        } else if i - UP_CHARS.len() - MIDDLE_CHARS.len() < DOWN_CHARS.len() {
            DOWN_CHARS[i - UP_CHARS.len() - MIDDLE_CHARS.len()]
        } else {
            panic!("Shouldn't happen");
        };

        assert_eq!(c, Some(actual_c));

        assert_eq!(all_chars.len(), remaining - 1);
        assert_eq!(all_chars.size_hint(), (remaining - 1, Some(remaining - 1)));
    }

    assert_eq!(all_chars.next(), None);
    assert_eq!(all_chars.next_back(), None);
    assert_eq!(all_chars.len(), 0);
    #[cfg(feature = "nightly")]
    assert!(all_chars.is_empty());
}


#[test]
fn char_kind() {
    let empty = CharKind::empty();

    let up = CharKind::UP;
    let middle = CharKind::MIDDLE;
    let down = CharKind::DOWN;

    let all = CharKind::all();

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

    assert_eq!(CharKind::from_bits(0b000), Some(empty));
    assert_eq!(CharKind::from_bits(0b001), Some(down));
    assert_eq!(CharKind::from_bits(0b010), Some(middle));
    assert_eq!(CharKind::from_bits(0b100), Some(up));
    assert_eq!(CharKind::from_bits(0b011), Some(middle | down));
    assert_eq!(CharKind::from_bits(0b101), Some(up | down));
    assert_eq!(CharKind::from_bits(0b110), Some(up | middle));
    assert_eq!(CharKind::from_bits(0b111), Some(all));

    assert_eq!(CharKind::from_bits(0b1011), None);
    assert_eq!(CharKind::from_bits(0b00100000), None);
}


#[test]
fn intensity() {
    let intensity = Intensity::Mini;  // Any value will do the job

    // Automatically detect new/removed variants by forcing compiler to use
    // exhaustiveness checks for enums.
    match intensity {
        Intensity::Mini => (),
        Intensity::Normal => (),
        Intensity::Maxi => (),
        Intensity::Random => (),
        Intensity::Custom { .. } => (),
    }
}


#[test]
fn apply_rng_iter() {
    let apply_compare = |text: &str, expected: &str, char_kind_bits, intensity| {
        assert!(
            zalgo::apply_rng_iter(
                rand::ChaChaRng::new_unseeded(),
                text.chars(),
                CharKind::from_bits(char_kind_bits).unwrap(),
                intensity,
            ).eq(expected.chars())
        );
    };

    let apply_batch = |intensity, expected_arr| {
        for &(char_kind_bits, expected) in &expected_arr {
            apply_compare("text", expected, char_kind_bits, intensity);
        }
    };

    apply_batch(Intensity::Mini, [
        (0b000, "text"),
        (0b001, "tex̰̺̯͖̱t͓͕̤"),
        (0b010, "t̵ex̨t"),
        (0b011, "t̵e̯̲̰̺̯xͅt͍̱͓͟"),
        (0b100, "te̓ͨ̇x̆̍͐t̓͌ͫ"),
        (0b101, "te̓ͨ̇x̰̆̍͐ͅṯͫ"),
        (0b110, "t̵eͨx͋̆̍͐ͧ͏t̨ͫ"),
        (0b111, "t̵e̲̰̺̯͖ͨx̢̱͓̺͗̅̓͌t̜̠͆͑̃͊̂̂̌ͅ"),
    ]);

    apply_batch(Intensity::Normal, [
        (0b000, "text"),
        (0b001, "t͕e̯̲̰x̱͓̰͓͕̤ͅt̺͍̞̙͇"),
        (0b010, "t̵͠ex̕͝t͏"),
        (0b011, "ṱ̵͠e͏̺̯͖x͏͓͕̤͍̱͓t̙͇̖̖͙̗̦̳"),
        (0b100, "t̓eͨx͋̆̍t̅"),
        (0b101, "t̮̓ė̱͓̓ͤ͋̆x͍̞̙͇̖̓͌ͫ̔ͭͮt̳̂"),
        (0b110, "t̵̓͠e̓ͤ͋̕x͗̅̓͟tͮ̽̈́̅͆͑̃͊"),
        (0b111, "t̵̝̓͠e͖̱ͤ͋x̤͍̅͟t͇̽̈́̅"),
    ]);

    apply_batch(Intensity::Maxi, [
        (0b000, "text"),
        (0b001, "t͕̮̭̝̯̲̰̺̯͖̱e͓͕̤͍̱͓̺͍̞̙͇x̗̦̳̜̠͕̟͔̱̣̼͔̩ͅt̼̯͕̤"),
        (0b010, "t̵e̴͏͏x̨t͟͟͟"),
        (0b011, "t̵̮̭̝̯̲̰̺̯͖̱͓e̤͍̱͓̺͍̞̙͇͟x̧̦̳̜̠͕̟͔̱̣̼͔̩̞ͅt̨̤̼͕̥̗́"),
        (0b100, "t̓͊̾̓ͨ̇̓ͤ͋̆̍e̅̓͌ͫ̔ͭͮx͆͑̃͊̂̂̌̀t̏̿̍͌̋"),
        (0b101, "t͓̰͓͕̤͍̱͓̺͍̓͊̾̓ͨ̇̓ͤ͋̆̍ͅe̘͉̼̯͕̤̼͕̥̗̻̤̲̙̙͔͑̃͊̂̂̌̀͛̒ͥ̏̿̍͌̋͂̈̈́x̠̮̫͙̥̼͖̩̝̞͓̖̺̻̲̞̳̠͂̊̒ͪ̐͑͊͐̒͗̋̎̇͊ͧt͎̖̯͇̦͔͔̣̼̳̠͙͂̅͌̎̔̂̆̏"),
        (0b110, "t̓͊̾̓ͨ̇̓ͤ͋̆̍͝e̡̓͌ͫ̔ͭx̨͆͑̃͊̂̂̌̀͟͟͞t̛̋͂̈̈́̉̃̾͒ͯ̓̋ͯ̔̎̑̎́̚͠͠"),
        (0b111, "t̰͓͕̤͍̱͓̺͍̞̓͊̾̓ͨ̇̓ͤ͋̆̍͝ͅẽ̡̼̯͕̤̼͕̥̗̻̤̲̙̙͔̙̫͇͊̂̂̌̀͛̒ͥ̏̿̍͌̋͂̈̈́͟x̧͙̥̼ͪ̐͑͊͐̒͗̋̎̇͊ͧ̒̀ẗ̡͉͍̭̮͎̖̯͇̦͔́ͭͪ͂̑̂̾̉ͨ͊͛ͣ͂̅͢"),
    ]);

    apply_batch(Intensity::Random, [
        (0b000, "text"),
        (0b001, "t̮̭̝e̯͖̱͓ͅx͍̱͓̺͍̞̙͇̖̖͙̗̦t͕̟͔̱̣̼͔"),
        (0b010, "text͏"),
        (0b011, "t̮̭̝e͖̱͓̰͝ͅx̨̡͍̞̙͇̖̖͙̗̦̳͘t̟͔̱"),
        (0b100, "t͊̾̓e͋̆̍͐ͧ͗̅̓xͮ̽̈́̅͆͑̃͊̂̂tͥ̏̿̍͌̋͂̈̈́̉"),
        (0b101, "t̯̲̰͊̾̓ḛ͓͕̤͐ͧx̠͕̟̽̈́̅͆͑̃͊̂̂̌̀͛t͉̼͂̈̈́̉"),
        (0b110, "t͊̾̓e͋̆̍͐ͧ͗̅̓͟x̽̈́̅͆͑̃͊̂̂̌̀͛͟͟͞t͂̈̈́̉͟"),
        (0b111, "t̯̲̰͊̾̓ḛ͓͕̤͐ͧx͔̱̣̽̈́̅͆͑̃͊̂̂̌̀͛͟͟͞t̸̨͕̥̗̻̤̲̙̙͔̙̫͇̉̃̾́͘"),
    ]);

    apply_batch(Intensity::Custom { up: 100, middle: 100, down: 100 }, [
        (0b000, "text"),
        (0b001, "t̖̙͎͕̮̭̝̯̲̰̺̯͖̱͓̰͓͕̤͍̱͓̺͍̞̙͇̖̖͙̗̦̳̜̠͕̟͔̱̣̼͔̩̞̘͉̼̯͕̤̼͕̥̗̻̤̲̙̙͔̙̫͇̺̪̠̱̱̪͇͈̞̲͍͙̠̮̫͙̥̼͖̩̝̞͓̖̺̻̲̞̳̠̟͙ͅͅͅͅͅe̪̜͓͈̗͉͍̭̮͎̖̯͇̦͔͔̣̼̳̠͙̯̪̗͖̪̘͙̜͇̝̤͈̳͈̯̹̰̟̬͍͎̮̞͚͎̠̹̪̯͕̖͙̟̬̲̪͚͈͕̬̯̟̪̫͈͍͖̭̠̗͍̪͙̗̟̟̺̟͚͉̟̜̙͖͉̯͉̩̙̩̭̗̟̩͙̟ͅͅͅx̳͎̥͕̦̦̮̟̥̰̳̙̩͙̗̝͎̖̼͓͖͓͕̖̣͔̣͔̼̜͚̺̞̙̻͓̟̘͙̱̳̯̠̺̙͍̩̖͉̞̜̮͕̼̯͚̝͇̰̣̤̻͚̰̠̦̺̭͍̞̪̺̜͕̥̟̥̗̹̺̮̥͇͔̥̫͖̺̘͚̳̰̦̦̱͔̺ͅͅͅt̤̩̭͉̬̻͉͔̹̝͇̳̞̺̙̱̰̹̱̱̠̯͔̰̘͚̪̣̲̫̬̳̦͙̪͙̞̱̥͍̭͖̣̦̝̫͚̺̜̙̫̙̪͈̗̖̬̖̥̳͓̰̹͈̭̭̮̗͉̙͔͉͍͔̭͚̼̯͔̘̟͖͕͇̫̰̤͓̼̱͕̫͈̯̹̩̭̩͙ͅ"),
        (0b010, "ţ̵̵̴̧͠͝͠͏͏̨̨̛̕͝͝͏̴̢̨̡̡̡̨̧̧̧̨̡͘̕̕͟͟͟͜͟͟͡͞͠͏̷҉̡̕͜͟҉̸̨́͘̕̕͏̶̵̴̷̨̧̛́̀͢͝͠͠͝͏̵̴́̕͢͜͠҉͝͏̷̧̢̧̢̢̧̡̀̀́͜͡͠͠͏̢̢͠é̷̡̧̛́̕͢͢͢͠͝͏̵̶̶̴̸̶̡̡̛̀́̀́͘̕͘͟͟͢͢͟͢͜͟͜͡͠҉̴͟҉̵̶̴̡̢̢̀͘͘͟͠͞͝͝͏̢̢͝͞͏̴̴̶̸̢̢̡̢̡̢̨̧̨̢̛̀́̀̀͘̕̕͘̕͘̕̕͢͟͟͜͞͏̷̢̡x҉̡͜͏̷̡͞͏̸͘͞҉̶̷̸̡̨͠҉̷̢͞͏̴̵̴̀͘͜͜͡͡҉̷̢̡̡̛͢͠͡͏́́҉̵̷̵̸̨̀͟͠͝͏̵̵̡̨̛̛́͝҉̸̴̵̸̴̷̵̶̵̵̨̨̧̛́̀̀́̀̀́̕͘͘͘͟͜͟͟͢͞͡͞͠͞͞t̸̢́̀͜͟͏̧̕͟҉̵̶̸̧̡̨̛͟͠͠҉͏̶̢̀́͘̕͡͞͏̵̴̴̧͟͝͞͏̸̶͟͠҉̴̶̶̨̢̛̛̛́͠͝͏̵͞͏̶̴̴̴̧̧̧̢̨́̀̕͢͟͝͡͝͝͏̴͢҉̧́̀͡͏̷̷̷̶̨̡̨̛͢͜͡͞͝"),
        (0b011, "ţ̵̵̴̧͠͝͠͏͏̨̨̛̕͝͝͏̴̢̨̡̡̡̨̧̧̧̨̡͘̕̕͟͟͟͜͟͟͡͞͠͏̷҉̡̕͜͟҉̸̨́͘̕̕͏̶̵̴̷̨̧̛́̀͢͝͠͠͝͏̵̴́̕͢͜͠҉͝͏̷̧̢̧̢̢̧̡̀̀́͜͡͠͠͏̢̢̪̜͓͈̗͉͍̭̮͎̖̯͇̦͔͔̣̼̳̠͙̯̪̗͖̪̘͙̜͇̝̤͈̳͈̯̹̰̟̬͍͎̮̞͚͎̠̹̪̯͕̖͙̟̬̲̪͚͈͕̬̯̟̪̫͈͍͖̭̠̗͍̪͙̗̟̟̺̟͚͉̟̜̙͖͉̯͉̩̙̩̭̗̟̩͙̟͠ͅͅͅe҉̡͜͏̷̡͞͏̸͘͞҉̶̷̸̡̨͠҉̷̢͞͏̴̵̴̀͘͜͜͡͡҉̷̢̡̡̛͢͠͡͏́́҉̵̷̵̸̨̀͟͠͝͏̵̵̡̨̛̛́͝҉̸̴̵̸̴̷̵̶̵̵̨̨̧̛̤̩̭͉̬̻͉͔̹̝͇̳̞̺̙̱̰̹̱̱̠̯͔̰̘͚̪̣̲̫̬̳̦͙̪͙̞̱̥͍̭͖̣̦̝̫͚̺̜̙̫̙̪͈̗̖̬̖̥̳͓̰̹͈̭̭̮̗͉̙͔͉͍͔̭͚̼̯͔̘̟͖͕͇̫̰̤͓̼̱͕̫͈̯̹̩̭̩͙́̀̀́̀̀́̕͘͘͘͟͜͟͟͢͞͡͞͠͞͞ͅx̷̴̷̴̸̢̢̨͘̕͜͢͠͠҉̶̷̷̵̧̡̨̢̛̕͜͠͞͡͝҉҉҉҉̶̷̶̢̨̛͢͠͝͝͠͝͏҉̴̶̴̶̛̀̀́͘̕͟͟͜͟͏̶̵̷̡̡̡̛̛̛̀͘͠͏̢̨̀͢͞͏̴̵̷̨̧̛͢͜͢͝͏̸̧̛̻͚̭̭̬̻̼̤̝̪̹̮͈̻̲̦͖̮̤̻̞͉̥̟̼͎͉̬̳̥̳͉̜̤̥̳͕͕̠̮͎͉̥͎̖͍͔̱̞̳̗̝̺̙͓͎̻̹͎̖̤̖͔͔̖̺̝͍͍̙̥̬̹͈̟͍̻̩̲͚͓͍̲̣̭̗̲͕̬͙̞̰̻̼̘͉̼́̕͢͠ͅͅͅt͏̴̀̀͜͏҉̵͏̸̷́́̀̕͢҉͏̵̢̧̨͘͜͝͠͝҉̷̸̴̴̵̶̵̶̴̷̨̨̡̢̀́̀́̕̕͘͢͢͟͟͜͡͝͝҉̶̸̷̵̵̴̴̸̨̡̧̨̢̧̨̛̕̕͜͟͢͟͡͠͝͡҉̴̵̶̢̢̧̨̛̛̩̮̯͔̭̟̥̥̼̰̺̮̣̫̦͚̖̲̞͇͈͉̫͕̣̘͔̖̖̜̲̠̣̦̲̬̘̬̘̠̙̠̝̼̗̥̙̟̲̹̠̞̫̞̻̼̼͔̟͉̳͍̥͎̤̘̣̗͓̦̙̦͇̹̹͈͚̣̬̻͙͉̹̻͈̘̫̳͉̞̺̤̫̼̟̫̞̻̱͘̕͘͘͜͜ͅ"),
        (0b100, "tͪ͐̉̓͊̾̓ͨ̇̓ͤ͋̆̍͐ͧ͗̅̓͌ͫ̔ͭͮ̽̈́̅͆͑̃͊̂̂̌̀͛̒ͥ̏̿̍͌̋͂̈̈́̉̃̾͒ͯ̓̋ͯ̔̎̑̎ͣ͐̿͐ͤ̒͂̊̒ͪ̐͑͊͐̒͗̋̎̇͊ͧ̒͊̿̏̃̈́̈́ͬͥ̈́ͭͪ͂̑̂̾̉ͨ͊͛̚eͣ͂̅͌̎̔̂̆̏̌̃͋̒̎ͤ͂ͦ̌̒̐͋͐̇̆͐ͬͧͮ̒ͯ͌̈͌̒̊͌͛̑ͫͬ̐̆͒̄͋̅̅͋̓̍̐̏́̇ͣ͋ͩ̑ͯͨ͊͐̀ͩ̎̾̆ͨ̎̂ͣ̐̎͊ͧ̀ͧ͒̍̐̿͐ͦͪ͒̐̃͊̉̊̄̈́̇͊̐̄͊͊̚̚x̌̉̔́ͫ̎͛͊͑͗̌̅̈ͧ̇́ͬ̃̋͐ͦͣͯ̔ͩ̀ͩͮ̆̿̒͂̾ͭ́̅̐͛̉̐ͪ̈͋ͨ̀̊̂͊̈ͪ͑̆̿͊́̆͋̒ͯ͆̓ͯͨ͆̎ͮ̆ͫͦͭ̿ͤ̑̔̐̔͛ͫͭ͂͛̔ͨͮͪ̿̾̿̌̒ͬͩͫ̽̍̿̿̚̚̚t͌̌ͦ̍ͥͥ͑̿̅͊̓̒̄̾̀ͣ̔͌ͣ͑̍͆ͨ̀͌̄͋ͣ͗̽ͤͯ̈ͫ͛͐ͧ̋ͪ̍̂̾ͦͩ̇́ͤ͋ͮ͂ͣ͂̊̊͗̇̔̑̍ͪ̉̅͗͐̓ͦ̆̏̇̍̅̀̍̇ͮ̈́͆̈́͆̀̄ͧ̆́͒ͮͩ͌̅̋̍̑̀̒̊̄ͦ̈͛̚"),
        (0b101, "t̪̜͓͈̗͉͍̭̮͎̖̯͇̦͔͔̣̼̳̠͙̯̪̗͖̪̘͙̜͇̝̤͈̳͈̯̹̰̟̬͍͎̮̞͚͎̠̹̪̯͕̖͙̟̬̲̪͚͈͕̬̯̟̪̫͈͍͖̭̠̗͍̪͙̗̟̟̺̟͚͉̟̜̙͖͉̯͉̩̙̩̭̗̟̩͙̟ͪ͐̉̓͊̾̓ͨ̇̓ͤ͋̆̍͐ͧ͗̅̓͌ͫ̔ͭͮ̽̈́̅͆͑̃͊̂̂̌̀͛̒ͥ̏̿̍͌̋͂̈̈́̉̃̾͒ͯ̓̋ͯ̔̎̑̎ͣ͐̿͐ͤ̒͂̊̒ͪ̐͑͊͐̒͗̋̎̇͊ͧ̒͊̿̏̃̈́̈́ͬͥ̈́ͭͪ͂̑̂̾̉ͨ͊͛̚ͅͅͅě̤̩̭͉̬̻͉͔̹̝͇̳̞̺̙̱̰̹̱̱̠̯͔̰̘͚̪̣̲̫̬̳̦͙̪͙̞̱̥͍̭͖̣̦̝̫͚̺̜̙̫̙̪͈̗̖̬̖̥̳͓̰̹͈̭̭̮̗͉̙͔͉͍͔̭͚̼̯͔̘̟͖͕͇̫̰̤͓̼̱͕̫͈̯̹̩̭̩͙̉̔́ͫ̎͛͊͑͗̌̅̈ͧ̇́ͬ̃̋͐ͦͣͯ̔ͩ̀ͩͮ̆̿̒͂̾ͭ́̅̐͛̉̐ͪ̈͋ͨ̀̊̂͊̈ͪ͑̆̿͊́̆͋̒ͯ͆̓ͯͨ͆̎ͮ̆ͫͦͭ̿ͤ̑̔̐̔͛ͫͭ͂͛̔ͨͮͪ̿̾̿̌̒ͬͩͫ̽̍̿̿̚̚̚ͅx̻͚̭̭̬̻̼̤̝̪̹̮͈̻̲̦͖̮̤̻̞͉̥̟̼͎͉̬̳̥̳͉̜̤̥̳͕͕̠̮͎͉̥͎̖͍͔̱̞̳̗̝̺̙͓͎̻̹͎̖̤̖͔͔̖̺̝͍͍̙̥̬̹͈̟͍̻̩̲͚͓͍̲̣̭̗̲͕̬͙̞̰̻̼̘͉̼͌͐̄̔ͣ̋͆ͮ͂̏ͧ̄͒ͤ̆̍̈̎ͪ̎̾͋̿ͬ̔̐̽́̏ͣ͑̅ͩ̊̈́̋̄ͮͧ̎̎ͨͪ̽͊ͯ̏ͧ͌͒̔ͤ̈́ͬͥ͑̊̈́͐͋̒̀̈̃̔ͧͬͧ̋̍̎̉͗̎͆ͮ̏͂̓̍ͨ̌̋ͨͭ̌̾̓̏ͤ͊͑̋̐̐̉̚̚̚̚ͅͅͅt̩̮̯͔̭̟̥̥̼̰̺̮̣̫̦͚̖̲̞͇͈͉̫͕̣̘͔̖̖̜̲̠̣̦̲̬̘̬̘̠̙̠̝̼̗̥̙̟̲̹̠̞̫̞̻̼̼͔̟͉̳͍̥͎̤̘̣̗͓̦̙̦͇̹̹͈͚̣̬̻͙͉̹̻͈̘̫̳͉̞̺̤̫̼̟̫̞̻̱͋̃ͭͨ̇̍̿͂̍ͮ͊̾̐̀͂̀͂̓̎͋̿̓ͤ̾̽̓̅͆͋͐̆̄͛ͣ̌ͫ͌ͩͨ̀́̌ͩ̒̿ͯ̓ͥ̍̐͑͂̎̾͋̏ͩ̔̌͊ͪ͆́ͣͯͩ͆̓́̇̆͌̉̐͊̍̅̍͒̾͑͑ͩͤ͑̏ͫ̈́̃̐̋̇̄ͯ́̽ͦͯ̚̚ͅ"),
        (0b110, "t̷̡̧̛ͪ͐̉̓͊̾̓ͨ̇̓ͤ͋̆̍͐ͧ͗̅̓͌ͫ̔ͭͮ̽̈́̅͆͑̃͊̂̂̌̀͛̒ͥ̏̿̍͌̋͂̈̈́̉̃̾͒ͯ̓̋ͯ̔̎̑̎ͣ͐̿͐ͤ̒͂̊̒ͪ̐͑͊͐̒͗̋̎̇͊ͧ̒͊̿̏̃̈́̈́ͬͥ̈́ͭͪ͂̑̂̾̉ͨ͊͛́́̚̕͢͢͢͠͝͏̵̶̶̴̸̶̡̡̛̀́̀́͘̕͘͟͟͢͢͟͢͜͟͜͡͠҉̴͟҉̵̶̴̡̢̢̀͘͘͟͠͞͝͝͏̢̢͝͞͏̴̴̶̸̢̢̡̢̡̢̨̧̨̢̛̀́̀̀͘̕̕͘̕͘̕̕͢͟͟͜͞͏̷̢̡ě̸̢̉̔́ͫ̎͛͊͑͗̌̅̈ͧ̇́ͬ̃̋͐ͦͣͯ̔ͩ̀ͩͮ̆̿̒͂̾ͭ́̅̐͛̉̐ͪ̈͋ͨ̀̊̂͊̈ͪ͑̆̿͊́̆͋̒ͯ͆̓ͯͨ͆̎ͮ̆ͫͦͭ̿ͤ̑̔̐̔͛ͫͭ͂͛̔ͨͮͪ̿̾̿̌̒ͬͩͫ̽̍̿̿́̀̚̚̚͜͟͏̧̕͟҉̵̶̸̧̡̨̛͟͠͠҉͏̶̢̀́͘̕͡͞͏̵̴̴̧͟͝͞͏̸̶͟͠҉̴̶̶̨̢̛̛̛́͠͝͏̵͞͏̶̴̴̴̧̧̧̢̨́̀̕͢͟͝͡͝͝͏̴͢҉̧́̀͡͏̷̷̷̶̨̡̨̛͢͜͡͞͝x̵̵̶̴̡̢̢͌͐̄̔ͣ̋͆ͮ͂̏ͧ̄͒ͤ̆̍̈̎ͪ̎̾͋̿ͬ̔̐̽́̏ͣ͑̅ͩ̊̈́̋̄ͮͧ̎̎ͨͪ̽͊ͯ̏ͧ͌͒̔ͤ̈́ͬͥ͑̊̈́͐͋̒̀̈̃̔ͧͬͧ̋̍̎̉͗̎͆ͮ̏͂̓̍ͨ̌̋ͨͭ̌̾̓̏ͤ͊͑̋̐̐̉́́̀́̚̚̚̚̕͘͘̕͜͟͟͢͢͜͞͝҉͜͜͠҉̵̴̶̡̀͘͢͜͡͡͏̢̧̕͜͏̴̡̧̢̧̛̀̀͘͟͡͞͏̷̵̢̛͘̕͟͢͢͝҉̷̶̴̨̢̛̀͘͟͟͢͡͏̵̵̸̨̢̢̢̛̀́͘̕͝͝͝͏̵̸̢ţ̵̸̵̵̴̸̸̸̸̴̧̢̨̧̧̢̛̛̛̛̛͋̃ͭͨ̇̍̿͂̍ͮ͊̾̐̀͂̀͂̓̎͋̿̓ͤ̾̽̓̅͆͋͐̆̄͛ͣ̌ͫ͌ͩͨ̀́̌ͩ̒̿ͯ̓ͥ̍̐͑͂̎̾͋̏ͩ̔̌͊ͪ͆́ͣͯͩ͆̓́̇̆͌̉̐͊̍̅̍͒̾͑͑ͩͤ͑̏ͫ̈́̃̐̋̇̄ͯ́̽ͦͯ́̀̀̀̀́̀̀́̚̚̕͘͘͘͜͟͢͢͟͢͜͟͟͟͡͡͡͠͠҉̵̷̶̧̡́̀̀̀̕͢͟͟͞͠͠͠͏̵̷̧̛̛̕͝͠͡҉͏̷̸̸̶̴̷̧̨̢̛̛̛̀͢͝͠͡҉̡͜"),
        (0b111, "t̷̡̧̛ͪ͐̉̓͊̾̓ͨ̇̓ͤ͋̆̍͐ͧ͗̅̓͌ͫ̔ͭͮ̽̈́̅͆͑̃͊̂̂̌̀͛̒ͥ̏̿̍͌̋͂̈̈́̉̃̾͒ͯ̓̋ͯ̔̎̑̎ͣ͐̿͐ͤ̒͂̊̒ͪ̐͑͊͐̒͗̋̎̇͊ͧ̒͊̿̏̃̈́̈́ͬͥ̈́ͭͪ͂̑̂̾̉ͨ͊͛́́̚̕͢͢͢͠͝͏̵̶̶̴̸̶̡̡̛̀́̀́͘̕͘͟͟͢͢͟͢͜͟͜͡͠҉̴͟҉̵̶̴̡̢̢̀͘͘͟͠͞͝͝͏̢̢͝͞͏̴̴̶̸̢̢̡̢̡̢̨̧̨̢̛̀́̀̀͘̕̕͘̕͘̕̕͢͟͟͜͞͏̷̢̡̳͎̥͕̦̦̮̟̥̰̳̙̩͙̗̝͎̖̼͓͖͓͕̖̣͔̣͔̼̜͚̺̞̙̻͓̟̘͙̱̳̯̠̺̙͍̩̖͉̞̜̮͕̼̯͚̝͇̰̣̤̻͚̰̠̦̺̭͍̞̪̺̜͕̥̟̥̗̹̺̮̥͇͔̥̫͖̺̘͚̳̰̦̦̱͔̺ͅͅͅe̷̴̷̴̸̢̢̨͌̌ͦ̍ͥͥ͑̿̅͊̓̒̄̾̀ͣ̔͌ͣ͑̍͆ͨ̀͌̄͋ͣ͗̽ͤͯ̈ͫ͛͐ͧ̋ͪ̍̂̾ͦͩ̇́ͤ͋ͮ͂ͣ͂̊̊͗̇̔̑̍ͪ̉̅͗͐̓ͦ̆̏̇̍̅̀̍̇ͮ̈́͆̈́͆̀̄ͧ̆́͒ͮͩ͌̅̋̍̑̀̒̊̄ͦ̈͛̚͘̕͜͢͠͠҉̶̷̷̵̧̡̨̢̛̕͜͠͞͡͝҉҉҉҉̶̷̶̢̨̛͢͠͝͝͠͝͏҉̴̶̴̶̛̀̀́͘̕͟͟͜͟͏̶̵̷̡̡̡̛̛̛̀͘͠͏̢̨̀͢͞͏̴̵̷̨̧̛͢͜͢͝͏̸̧̛̻͚̭̭̬̻̼̤̝̪̹̮͈̻̲̦͖̮̤̻̞͉̥̟̼͎͉̬̳̥̳͉̜̤̥̳͕͕̠̮͎͉̥͎̖͍͔̱̞̳̗̝̺̙͓͎̻̹͎̖̤̖͔͔̖̺̝͍͍̙̥̬̹͈̟͍̻̩̲͚͓͍̲̣̭̗̲͕̬͙̞̰̻̼̘͉̼́̕͢͠ͅͅͅx̵̸̵̵̴̸̸̸̸̴̧̧̢̨̧̧̢̛̛̛̛̛͋̃ͭͨ̇̍̿͂̍ͮ͊̾̐̀͂̀͂̓̎͋̿̓ͤ̾̽̓̅͆͋͐̆̄͛ͣ̌ͫ͌ͩͨ̀́̌ͩ̒̿ͯ̓ͥ̍̐͑͂̎̾͋̏ͩ̔̌͊ͪ͆́ͣͯͩ͆̓́̇̆͌̉̐͊̍̅̍͒̾͑͑ͩͤ͑̏ͫ̈́̃̐̋̇̄ͯ́̽ͦͯ́̀̀̀̀́̀̀́̚̚̕͘͘͘͜͟͢͢͟͢͜͟͟͟͡͡͡͠͠҉̵̷̶̧̡́̀̀̀̕͢͟͟͞͠͠͠͏̵̷̧̛̛̕͝͠͡҉͏̷̸̸̶̴̷̧̨̢̛̛̛̀͢͝͠͡҉̡̬̲̣̟̯̠̞̦͓͉̗͍͇̭̦̪͍̭̦̱̬͕͎̰̣̲̮̣͎̺͔͔̥͓̣̙̤͖̝̘͙̥̩͕̜̰̗͇̩̺͔̫͍̩̱̗͇͚̺̲̼͇̮͙̙͎͖͖̰̰͙͈̟͎̤̙̞͕̹̲̣̝͇̙̖̼̙̥̹͈̦͚̞̼̭̼̖͉̭͚͜t̸̴̵̵̡̧̢̛̾̒̾̌̓́́͂̆̓͊ͯ̍ͧ̿̌̃̓̈́͐̓̏͒̓̈̏̾̋͂͛͌̇̽ͯ̆̎̿ͣͭ̑ͩ̽̓ͬ͐̓̽̎͋͗̂̇ͫ̉̆̓̉ͫ̽̈́̓̃ͭͨͯ͊ͨ̃ͦ̑̂̈̂̋̀͐̀̎͆͌̐̌͒̓̇̎̆ͥͭ͒̒ͦ͋ͬ̌̒̍͛̀̀̀̚̚̕͘͢͢͜͜͞͞͠͝͡͏̡͜͏̸͘͘͢҉́̕͢͠͡҉͢͡͏̷͡͝͞͝͡͏̸̵̡́͘̕͝͞҉̸̵̶̸̶̴̸̵̨̢̢̢̛̛́̀̀͘̕̕̕͝͞͞͠͝҉̷̷̵̡̨̨̨̧͟͜҉̷̷̡̼̣̞̹̥͎̞̞̙̙̝̖͓̬̪̝̣̜̰̭̥͍̘͍͍͉̙̮̭̦͎̙̲̘̰̗̼͓͔̙̱͚̥̩̲̗̯̦̦͚̲̰͉̫̪̻̩̟͎͚̣̣̹̖͖͙̺̬̤̪̼̯̭̖̠̹̻̱̯̜͓͍͇͉͎̲͇͙̘͖̻͚̱͎̻͉͎̕͟͟ͅͅͅ"),
    ]);
}

#[cfg(feature = "std")]
#[test]
fn apply() {
    // It's not really possible to test the outputs, so just test whether they
    // work or not.
    let apply_batch = |intensity| {
        for char_kind_bits in 0b000..0b111 {
            let _ = zalgo::apply("t", CharKind::from_bits(char_kind_bits).unwrap(), intensity);
        }
    };

    apply_batch(Intensity::Mini);
    apply_batch(Intensity::Normal);
    apply_batch(Intensity::Maxi);
    apply_batch(Intensity::Random);
    apply_batch(Intensity::Custom { up: 100, middle: 100, down: 100 });

    // Test that passing a String works.
    let _ = zalgo::apply(&String::from("t"), CharKind::empty(), Intensity::Random);
}


// Small crash tests
macro_rules! do_roundtrips {
    (
        core => {
            $( $name_core:ident, $str_core:expr => $times_core:expr, )*
        }

        alloc => {
            $( $name_alloc:ident, $str_alloc:expr => $times_alloc:expr, )*
        }

        std => {
            $( $name_std:ident, $str_std:expr => $times_std:expr, )*
        }
    ) => {
        $(
            #[test]
            fn $name_core() {
                for _ in 0..$times_core {
                    assert!(
                        zalgo::unapply_iter(zalgo::apply_rng_iter(
                            rand::XorShiftRng::new_unseeded(),
                            $str_core.chars(),
                            CharKind::all(),
                            Intensity::Maxi,
                        )).eq($str_core.chars())
                    );
                }
            }
        )*

        $(
            #[cfg(any(feature = "std", feature = "alloc"))]
            #[test]
            fn $name_alloc() {
                for _ in 0..$times_alloc {
                    assert_eq!(
                        &zalgo::unapply(&zalgo::apply_rng(
                            rand::XorShiftRng::new_unseeded(),
                            $str_alloc,
                            CharKind::all(),
                            Intensity::Maxi,
                        )),
                        $str_alloc
                    );
                }
            }
        )*

        $(
            #[cfg(feature = "std")]
            #[test]
            fn $name_std() {
                for _ in 0..($times_std / 2) {  // Running 2 operations per iteration
                    assert!(
                        zalgo::unapply_iter(zalgo::apply_iter(
                            $str_core.chars(),
                            CharKind::all(),
                            Intensity::Maxi,
                        )).eq($str_core.chars())
                    );

                    assert_eq!(
                        &zalgo::unapply(&zalgo::apply(
                            $str_std,
                            CharKind::all(),
                            Intensity::Maxi,
                        )),
                        $str_std
                    );
                }
            }
        )*
    }
}

do_roundtrips! {
    core => {
        do_roundtrips_core_empty, "" => 100000,
        do_roundtrips_core_foo, "foo" => 300,
        do_roundtrips_core_zalgo_description, zalgo::DESCRIPTION => 50,
        do_roundtrips_core_lorem_ipsum, lipsum::LOREM_IPSUM => 15,
        do_roundtrips_core_liber_primus, lipsum::LIBER_PRIMUS => 1,
    }

    alloc => {
        do_roundtrips_alloc_empty, "" => 100000,
        do_roundtrips_alloc_foo, "foo" => 300,
        do_roundtrips_alloc_zalgo_description, zalgo::DESCRIPTION => 50,
        do_roundtrips_alloc_lorem_ipsum, lipsum::LOREM_IPSUM => 15,
        do_roundtrips_alloc_liber_primus, lipsum::LIBER_PRIMUS => 1,
    }

    std => {
        do_roundtrips_std_empty, "" => 100000,
        do_roundtrips_std_foo, "foo" => 300,
        do_roundtrips_std_zalgo_description, zalgo::DESCRIPTION => 50,
        do_roundtrips_std_lorem_ipsum, lipsum::LOREM_IPSUM => 15,
        do_roundtrips_std_liber_primus, lipsum::LIBER_PRIMUS => 1,
    }
}
