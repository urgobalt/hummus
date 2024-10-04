use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(whatever, display("{message}"))]
pub struct FatalError {
    #[snafu(source(from(MainError, Some)))]
    pub source: Option<MainError>,
    pub message: String,
    pub backtrace: Backtrace,
}

#[derive(Debug, Snafu)]
pub enum MainError {
    #[snafu(display("Invalid arguments"))]
    ArgumentError,
}
