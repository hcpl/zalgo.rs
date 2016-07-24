[travis-badge]: https://img.shields.io/travis/zeyla/zalgo.rs.svg?style=flat-square
[travis]: https://travis-ci.org/zeyla/zalgo.rs
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[license]: https://opensource.org/licenses/ISC

[![travis-badge][]][travis] [![license-badge][]][license]

# zalgo.rs

Rust crate for generating Zalgo text.


### Zalgo

> Zalgo is an Internet legend about an ominous entity believed to cause
> insanity, death and destruction of the world, similar to the creature Cthulhu
> created by H.P. Lovecraft in the 1920s. Zalgo is often associated with
> scrambled text on webpages and photos of people whose eyes and mouth have been
> covered in black.
>
> -- [knowyourmeme](http://knowyourmeme.com/memes/zalgo)


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

[examples]: https://github.com/zeyla/zalgo.rs/tree/master/examples
[LICENSE.md]: https://github.com/zeyla/zalgo.rs/blob/master/LICENSE.md
