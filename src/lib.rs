//! # Snakecase
//!
//! is a general purpose snakecase implementation supporting both ascii and unicode.
//!
//! **Notes:** Its algorithm is designed to provide feature parity with [this](https://github.com/segmentio/go-snakecase) Go snakecase library, but PR's will be accepted for other algorithms and can be hidden behind a feature flag.
//!
//! ```rust
//! use snakecase::ascii::to_snakecase;
//!
//! fn main() {
//!     let input = "sample text";
//!     println!("{} => {}", input, to_snakecase(input));
//! }
//! ```
//!
//! or when you need unicode support
//!
//! ```rust
//! use snakecase::unicode::to_snakecase;
//!
//! fn main() {
//!     let input = "ƒun sample text";
//!     println!("{} => {}", input, to_snakecase(input));
//! }
//! ```
//!

pub mod ascii;
pub mod unicode;
