#![feature(type_alias_impl_trait)]
// Safety-critical application lints
#![deny(
    bare_trait_objects,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::pedantic,
    clippy::result_unwrap_used
)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::empty_enum, clippy::iter_nth_zero, clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
mod error;
mod ports;
mod app;

use error::Error;
use iced::{Application, Settings};
use std::env;

type Result<T, E = Error> = std::result::Result<T, E>;

const PORT: u16 = 4444;

fn main() -> Result<()> {
    Ok(())
}
