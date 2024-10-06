#![feature(let_chains)]

use clap::Parser;
use directories::ProjectDirs;
use snafu::{ErrorCompat, ResultExt};

mod command_arguments;
mod database;
mod error;
mod log;
mod rust_backtrace;

pub use command_arguments::Args;
use error::MainError;
use rust_backtrace::RustBacktrace;
use tracing::{info, trace, warn, Value};

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
    let _guard = log::init(&args, &project_dirs);
    if let Some(project_dirs) = project_dirs {
        info!("Successfully found project directories");
        trace!("{:?}", project_dirs)
    } else {
        warn!("Unable to find project directories");
    }

    database::init(&args);
    Ok(())
}
