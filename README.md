[![ci-badge][]][ci] [![license-badge][]][license] [![docs-badge][]][docs]

# zalgo.rs

Rust crate for generating Zalgo text.

This features both a library and a binary, which can be used via:

`cargo run --bin zalgo test text`

### Zalgo

> Zalgo is an Internet legend about an ominous entity believed to cause
> insanity, death and destruction of the world, similar to the creature Cthulhu
> created by H.P. Lovecraft in the 1920s. Zalgo is often associated with
> scrambled text on webpages and photos of people whose eyes and mouth have been
> covered in black.
>
> -- [knowyourmeme][kym]

### Installation

Add the following dependency to your Cargo.toml:

```rust
zalgo = "0.1"
```

And include it in your project:

```rust
extern crate zalgo;
```

### Examples

Generate Zalgo text with up/down (no middle) and mini definitions:

```rust
use zalgo::ZalgoSize;

let text = zalgo::gen("test", true, false, true, ZalgoSize::Mini);
```

To view more examples, check out the [examples] directory.

### License

License info in [LICENSE.md]. Long story short, ISC.

[ci]: https://travis-ci.org/zeyla/zalgo.rs
[ci-badge]: https://travis-ci.org/zeyla/zalgo.rs.svg?branch=master
[docs]: https://docs.rs/zalgo
[docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg
[examples]: https://github.com/zeyla/zalgo.rs/tree/master/examples
[kym]: http://knowyourmeme.com/memes/zalgo
[LICENSE.md]: https://github.com/zeyla/zalgo.rs/blob/master/LICENSE.md
[license]: https://opensource.org/licenses/ISC
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
