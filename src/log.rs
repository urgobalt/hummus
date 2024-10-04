use crate::Args;
use directories::ProjectDirs;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

pub fn init(args: Args, project_dirs: Option<ProjectDirs>) -> Option<WorkerGuard> {
    let stdout = if args.stdout_log {
        Some(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true),
        )
    } else {
        None
    };

    let (log_file, guard) = if !args.disable_log_file
        && let Some(project_dirs) = project_dirs
    {
        let file_appender = tracing_appender::rolling::hourly(project_dirs.data_dir(), "main.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        (
            Some(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(non_blocking),
            ),
            Some(guard),
        )
    } else {
        (None, None)
    };

    tracing_subscriber::registry()
        .with(stdout)
        .with(log_file)
        .init();

    guard
}
