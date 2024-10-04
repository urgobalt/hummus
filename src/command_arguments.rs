use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
pub struct Args {
    /// Enable logging to stdout
    #[arg(short, long)]
    pub stdout_log: bool,

    /// Disable logging to the log file
    #[arg(short, long)]
    pub disable_log_file: bool,
}
