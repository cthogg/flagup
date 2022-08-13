//! This is the internal library for the
//! [`flagup`](https://github.com/cthogg/flagup) command line utility.
//!
//! Using the crate directly is not recommended since it is not maintained with a stable
//! API interface.
mod flag;

pub use self::flag::generate_flag_from_country;
