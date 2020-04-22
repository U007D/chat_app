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
mod app;
mod error;
use crate::app::App;
use error::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

//FYI to self impl std::error::Error for Error {} // implement trait for type, Error type is now impls std error trait

fn main() -> Result<()> {
    //    println!("Hello, {:?}", env::args().nth(1).ok_or(MissingNameArg)?);
    //    ChatWindow::run(Settings::default());
    let app = App::start()?;
    let _ = app
        .listener_thread
        .join()
        .map_err(|e| Error::AppStartError(e))?;
    Ok(())
}
