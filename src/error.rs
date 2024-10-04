use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
pub enum MainError {
    #[snafu(display("Invalid arguments"))]
    ArgumentError,
}
