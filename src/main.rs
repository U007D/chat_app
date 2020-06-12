#![feature(type_alias_impl_trait)]
// Safety-critical application lints
#![deny(
    bare_trait_objects,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::pedantic
)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::empty_enum, clippy::iter_nth_zero, clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]

mod adapters;
mod app;
mod error;
mod ports;

use error::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

const fn main() -> Result<()> {
    Ok(())
}
