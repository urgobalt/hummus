pub use snafu::{ErrorCompat, ResultExt};

mod error;
mod rust_backtrace;

use error::{FatalError, MainError};
use rust_backtrace::RustBacktrace;

#[tokio::main]
async fn main() {
    if let Err(err) = _main().await {
        if RustBacktrace::read().into() {
            if let Some(backtrace) = err.backtrace() {
                eprintln!("{:?}", backtrace);
            }
        }
        if let Some(source) = err.source {
            eprintln!("Failed with error: \x1b[31m{}\x1b[0m\n", source);
        }
        eprintln!("\x1b[92m{}\x1b[0m", err.message)
    }
}

async fn _main() -> Result<(), FatalError> {
    Err(MainError::ArgumentError).whatever_context("Clap failed parsing input")?;
    Ok(())
}
