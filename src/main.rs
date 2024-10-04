pub use snafu::{ErrorCompat, ResultExt};

mod error;
mod rust_backtrace;

use error::MainError;
use rust_backtrace::RustBacktrace;

#[tokio::main]
async fn main() {
    if let Err(err) = _main().await {
        if RustBacktrace::read().into() {
            if let Some(backtrace) = err.backtrace() {
                eprintln!("{:?}", backtrace);
            }
        }
        eprintln!("\x1b[31m{}\x1b[0m", err)
    }
}

async fn _main() -> Result<(), MainError> {
    Err(MainError::ArgumentError)
}
