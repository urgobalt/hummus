use std::path::PathBuf;

use clap::{Parser, Subcommand};

const DB_URL: &str = "HUMMUS_DB_URL";
const DB_TOKEN: &str = "HUMMUS_DB_TOKEN";

#[derive(Parser, Debug)]
pub struct Args {
    /// Enable logging to stdout
    #[arg(long = "stdout")]
    pub stdout_log: bool,

    /// Disable plain text logging to a file
    #[arg(long)]
    pub disable_log: bool,

    /// Disable json logging to a file
    #[arg(long)]
    pub disable_json_log: bool,

    #[command(subcommand)]
    pub command: DatabaseCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DatabaseCommand {
    Local {
        /// Path to the database file
        #[arg(short, long, env = DB_URL)]
        path: PathBuf,
    },
    Remote {
        /// The database connection url
        #[arg(short = 'u', long, env = DB_URL)]
        database_connection_url: String,

        /// The database authorization token (MUST BE KEPT SECRET)
        #[arg(short = 't', long, env = DB_TOKEN)]
        database_auth_token: String,
    },
}
