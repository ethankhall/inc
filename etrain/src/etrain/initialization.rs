extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::{Level, Drain, LevelFilter, Logger};
use std::env;

pub fn logging(min_level: Level, app_name: &str) -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = LevelFilter::new(drain, min_level).fuse();
    return Logger::root(drain, slog_o!("app" => app_name.to_string()));
}

pub fn log_level(number_of_verbose: u64) -> Level {
    return match number_of_verbose {
        0 => Level::Warning,
        1 => Level::Info,
        2 => Level::Debug,
        3 | _ => Level::Trace,
    };
}

pub fn log_from_env() -> Level {
    if let Ok(inherited_log_level) = env::var("ETRAIN_LOG_LEVEL") {
        if let Ok(value) = inherited_log_level.parse() {
            return log_level(value);
        }
    }

    return log_level(1);
}