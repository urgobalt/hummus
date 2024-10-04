#![feature(let_chains)]

use clap::Parser;
use directories::ProjectDirs;
use snafu::{ErrorCompat, ResultExt};

mod command_arguments;
mod error;
mod log;
mod rust_backtrace;

pub use command_arguments::Args;
use error::MainError;
use rust_backtrace::RustBacktrace;

const QUALIFIER: &str = "";
const ORGANISATION: &str = "";

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
    let args = Args::parse();
    let project_dirs = ProjectDirs::from(QUALIFIER, ORGANISATION, clap::crate_name!());
    let _guard = log::init(args, project_dirs);
    Ok(())
}
