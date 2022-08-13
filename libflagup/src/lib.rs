//! This is the internal library for the
//! [`when`](https://github.com/cthogg/flagup) command line utility.
//!
//! Using this crate directly is not recommended as it's not maintained with a stable
//! API interface.
mod flag;

pub use self::flag::generate_flag_from_country;
