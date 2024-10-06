use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
pub enum MainError {
    #[snafu(transparent)]
    DatabaseError { source: libsql::errors::Error },
}
