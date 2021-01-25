# Snakecase &emsp; ![Build Status] [![Latest Version]][crates.io]

[Build Status]: https://github.com/rust-playground/snakecase/workflows/Lint%20&%20Test/badge.svg 
[Latest Version]: https://img.shields.io/crates/v/snakecase.svg
[crates.io]: https://crates.io/crates/snakecase

**Snakecase** is a general purpose snakecase implementation supporting both ascii and unicode.

**Notes:** Its algorithm is designed to provide feature parity with [this](https://github.com/segmentio/go-snakecase) Go snakecase library, but PR's will be accepted for other algorithms and can be hidden behind a feature flag.

---

```toml
[dependencies]
snakecase = "0.1"
```

## Example usages
```rust
use snakecase::ascii::to_snakecase;

fn main() {
    let input = "sample text";
    println!("{} => {}", input, to_snakecase(input));
}

```

or when you need unicode support

```rust
use snakecase::unicode::to_snakecase;

fn main() {
    let input = "ƒun sample text";
    println!("{} => {}", input, to_snakecase(input));
}


```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Snakecase by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
