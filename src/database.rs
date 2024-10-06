use crate::{error::MainError, Args};
use libsql::Builder;

pub async fn init(args: &Args) -> Result<(), MainError> {
    let db = match args.command.clone() {
        crate::command_arguments::DatabaseCommand::Local { path } => {
            Builder::new_local(path).build().await
        }
        crate::command_arguments::DatabaseCommand::Remote {
            database_connection_url,
            database_auth_token,
        } => {
            Builder::new_remote(database_connection_url, database_auth_token)
                .build()
                .await
        }
    }?;

    Ok(())
}
