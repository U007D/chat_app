#![allow(clippy::pub_enum_variant_names)]
mod app;
pub mod transport;

pub use self::app::{Error, Result};
