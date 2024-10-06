use crate::Args;
use directories::ProjectDirs;
use tracing::{info, warn};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

pub fn init(
    args: &Args,
    project_dirs: &Option<ProjectDirs>,
) -> (Option<WorkerGuard>, Option<WorkerGuard>) {
    let mut logger_enabled = false;

    let stdout = if args.stdout_log {
        let out = Some(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true),
        );
        logger_enabled = true;
        out
    } else {
        None
    };

    let (log_json, guard) = if !args.disable_json_log
        && let Some(project_dirs) = project_dirs
    {
        let file_appender = tracing_appender::rolling::hourly(project_dirs.data_dir(), "json.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let out = (
            Some(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_ids(true)
                    .with_writer(non_blocking),
            ),
            Some(guard),
        );
        logger_enabled = true;
        out
    } else {
        (None, None)
    };

    let (log_file, guard2) = if !args.disable_log
        && let Some(project_dirs) = project_dirs
    {
        let file_appender = tracing_appender::rolling::hourly(project_dirs.data_dir(), "plain.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let out = (
            Some(
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_ids(true)
                    .with_writer(non_blocking),
            ),
            Some(guard),
        );
        logger_enabled = true;
        out
    } else {
        (None, None)
    };

    tracing_subscriber::registry()
        .with(stdout)
        .with(log_json)
        .with(log_file)
        .init();

    if logger_enabled {
        info!("Successfully initialized logger");
    } else {
        warn!("No logger is attached!");
    }

    (guard, guard2)
}
