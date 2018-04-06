#[cfg(all(not(feature = "std"), any(feature = "nightly", stable_fused_iterator)))]
use core::iter;

#[cfg(all(feature = "std", any(feature = "nightly", stable_fused_iterator)))]
use std::iter;

use rand::Rng;

use {UP_CHARS, MIDDLE_CHARS, DOWN_CHARS, CharKind, Intensity, is_zalgo};


/// Returns an [`Iterator`] of `char`s of generated Zalgo text with
/// user-provided random generator.
///
/// The output is customizable via defining whether to include Zalgo text above
/// the given string, in the middle of it, and below it.
///
/// The amount of Zalgo text can be (more or less) defined by the value of the
/// `intensity` given. Read on the [`Intensity`] for more information.
///
/// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
/// [`Intensity`]: enum.Intensity.html
pub fn apply_rng_iter<R: Rng, I: Iterator<Item = char>>(
    rng: R,
    chars: I,
    kind: CharKind,
    intensity: Intensity,
) -> ApplyRngIter<R, I> {
    ApplyRngIter { rng, chars, kind, intensity, state: State::Free }
}

/// An iterator of `char`s that are output as `char`s from Zalgo-transformed
/// text.
///
/// This struct is created by the [`apply_rng_iter`] function. See its
/// documentation for more details.
///
/// [`apply_rng_iter`]: fn.apply_rng_iter.html
pub struct ApplyRngIter<R, I> {
    rng: R,
    chars: I,
    kind: CharKind,
    intensity: Intensity,

    state: State,
}

enum State {
    Free,
    GenUp {
        count_up: usize,
        count_mid: usize,
        count_down: usize,
    },
    GenMid {
        count_mid: usize,
        count_down: usize,
    },
    GenDown {
        count_down: usize,
    },
    Finished,
}

fn generate_counts<R: Rng>(rng: &mut R, intensity: Intensity) -> (usize, usize, usize) {
    match intensity {
        Intensity::Mini => (
            rng.gen_range(0, 8),
            rng.gen_range(0, 2),
            rng.gen_range(0, 8),
        ),
        Intensity::Normal => (
            rng.gen_range(0, 16) / 2 + 1,
            rng.gen_range(0, 6) / 2,
            rng.gen_range(0, 16) / 2 + 1,
        ),
        Intensity::Maxi => (
            rng.gen_range(0, 64) / 4 + 3,
            rng.gen_range(0, 16) / 4 + 1,
            rng.gen_range(0, 64) / 4 + 3,
        ),
        Intensity::Random => unreachable!(),
    }
}

impl<R, I> Iterator for ApplyRngIter<R, I>
    where R: Rng,
          I: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.state {
            State::Free => {
                while let Some(ch) = self.chars.next() {
                    // Skip the text if it's already a Zalgo char
                    if is_zalgo(ch) {
                        continue;
                    }

                    let (count_up, count_mid, count_down) = if self.intensity == Intensity::Random {
                        let choices = [Intensity::Mini, Intensity::Normal, Intensity::Maxi];
                        let choice = *self.rng.choose(&choices).unwrap();

                        generate_counts(&mut self.rng, choice)
                    } else {
                        generate_counts(&mut self.rng, self.intensity)
                    };

                    self.state = if self.kind.contains(CharKind::UP) && count_up > 0 {
                        State::GenUp { count_up, count_mid, count_down }
                    } else if self.kind.contains(CharKind::MIDDLE) && count_mid > 0 {
                        State::GenMid { count_mid, count_down }
                    } else if self.kind.contains(CharKind::DOWN) && count_down > 0 {
                        State::GenDown { count_down }
                    } else {
                        State::Free
                    };

                    return Some(ch);
                }

                self.state = State::Finished;

                None
            },
            State::GenUp { count_up, count_mid, count_down } => {
                debug_assert!(count_up > 0);
                let c = *self.rng.choose(&UP_CHARS).unwrap();

                self.state = if count_up == 1 {
                    if self.kind.contains(CharKind::MIDDLE) && count_mid > 0 {
                        State::GenMid { count_mid, count_down }
                    } else if self.kind.contains(CharKind::DOWN) && count_down > 0 {
                        State::GenDown { count_down }
                    } else {
                        State::Free
                    }
                } else {
                    State::GenUp { count_up: count_up - 1, count_mid, count_down }
                };

                Some(c)
            },
            State::GenMid { count_mid, count_down } => {
                debug_assert!(count_mid > 0);
                let c = *self.rng.choose(&MIDDLE_CHARS).unwrap();

                self.state = if count_mid == 1 {
                    if self.kind.contains(CharKind::DOWN) && count_down > 0 {
                        State::GenDown { count_down }
                    } else {
                        State::Free
                    }
                } else {
                    State::GenMid { count_mid: count_mid - 1, count_down }
                };

                Some(c)
            },
            State::GenDown { count_down } => {
                debug_assert!(count_down > 0);
                let c = *self.rng.choose(&DOWN_CHARS).unwrap();

                self.state = if count_down == 1 {
                    State::Free
                } else {
                    State::GenDown { count_down: count_down - 1 }
                };

                Some(c)
            },
            State::Finished => {
                None
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (chars_lower, _) = self.chars.size_hint();

        (chars_lower, None)
    }
}

#[cfg(any(feature = "nightly", stable_fused_iterator))]
impl<R, I> iter::FusedIterator for ApplyRngIter<R, I>
    where R: Rng,
          I: Iterator<Item = char>,
{}
