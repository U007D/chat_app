mod app;
mod error;
use error::Error;
use crate::app::App;

type Result<T, E = Error> = std::result::Result<T, E>;

//FYI to self impl std::error::Error for Error {} // implement trait for type, Error type is now impls std error trait

fn main() -> Result<()> {
//    println!("Hello, {:?}", env::args().nth(1).ok_or(MissingNameArg)?);
//    ChatWindow::run(Settings::default());
    let app = App::start()?;
    let _ = app.listener_thread.join().map_err(|e| Error::AppStartError(e));
    Ok(())
}
