# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- Static variables that contain all 3 types of Zalgo chars: `UP_CHARS`, `MIDDLE_CHARS`,
  `DOWN_CHARS`;
- `all_chars` function which returns `AllChars` — an iterator that yields all Zalgo chars.
- Three functions of `apply*` family:
  * `apply_rng_iter` operates on an arbitrary `R: Rng` and `I: Iterator<Item = char>` and returns
    `ApplyRngIter<R, I>` — an `Iterator<Item = char>` that wraps the provided RNG and iterator of
    chars;
  * `apply_rng` equivalent to `apply` but also accepts a `R: Rng` as the first parameter;
  * `apply_iter` equivalent to `apply_rng_iter` except without the first `R: Rng` parameter and uses
    `ThreadRng` internally like `apply`; returns `ApplyRngIter<ThreadRng, I>`.
- Two functions of `unapply*` family:
  * `unapply` which removes Zalgo chars from the input `&str` and returns a `String`;
  * `unapply_iter` which removes Zalgo chars from the input `Iterator<Item = char>` and returns
    `UnapplyIter` — another `Iterator<Item = char>` that wraps the original one.
- Zalgo description text, placed in `zalgo::DESCRIPTION` static variable.
- `no_std` compatibility along with new Cargo features `std` and `alloc` where the latter can only
  be used on nightly due to `alloc` crate being unstable and behind the `#![feature(alloc)]` flag;
- `nightly` Cargo feature flag for opting into using nightly features.

### Changed

- Renames:
  * An attempt to remove redundant the `Zalgo` prefix from public items was made, thus:
    + `ZalgoKind` => `CharKind` (which is now a `bitflags` struct, rather than an enum);
    + `ZalgoSize` => `Intensity`;
  * `gen` => `apply`.
- Performance upgrades:
  * `is_zalgo` now instead of `all` internally uses `all_chars` which doesn't allocate on heap.
- Other:
  * Previously `gen` and then `apply` now takes 3 parameters: input `&str` text, `CharKind` bitflags
    and an `Intensity` variant;
  * `zalgo` (specifically, `ApplyRngIter` under the hood) now uses the algorithm from
    <http://eeemo.net>;
  * Aside from renaming, `Intensity` removed the `None` variant and instead added `Normal`, `Random`
    and `Custom`.
  * `zalgo` binary has benn significantly modified using `structopt` and `colored` to provide a
    pleasant CLI experience.

### Removed

- `all` function which returned a `Vec` of Zalgo chars, replaced with an `Iterator`-based
  `all_chars` function; if you need the old behaviour, you can do `all_chars().collect()`.

## [0.1.1] - 2016-08-21

### Added

### Changed

- Optimize and simplify the codebase

## [0.1.0] - 2016-02-28

Initial commit.

[Unreleased]: https://github.com/zeyla/zalgo.rs/compare/v0.1.1...master
[0.1.1]: https://github.com/zeyla/zalgo.rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/zeyla/zalgo.rs/tree/v0.1.0
