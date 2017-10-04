extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog_async::Async;
use slog::{Level, Logger, LevelFilter, Drain};
use slog_term::{CompactFormat, FullFormat, TermDecorator};
use std::env;
use std::fmt::Debug;

pub fn get_verbosity_level() -> Level {
    let args_level = parse_from_args();
    let env_level = parse_from_env();

    if args_level.as_usize() > env_level.as_usize() {
        return args_level;
    } else {
        return env_level;
    }
}

pub fn logging(min_level: Level, app_name: &String) -> Logger {
    let decorator = TermDecorator::new().stdout().build();
    let logger = if min_level == Level::Debug || min_level == Level::Trace {
        let format = FullFormat::new(decorator);
        build_with_drain(min_level, format.build())
    } else {
        let format = CompactFormat::new(decorator);
        build_with_drain(min_level, format.build())
    };

    slog_debug!(
        logger,
        "Starting app {}, version: {}",
        app_name,
        env!("CARGO_PKG_VERSION")
    );
    return logger;
}

fn build_with_drain<D>(min_level: Level, drain: D) -> Logger
where
    D: Drain + Send + 'static,
    D::Err: Debug,
{
    let drain = Async::new(drain.fuse()).chan_size(1024).build().fuse();
    let drain = LevelFilter::new(drain, min_level).fuse();
    Logger::root(drain, o!())
}

pub fn log_level(number_of_verbose: u64) -> Level {
    return match number_of_verbose {
        0 => Level::Warning,
        1 => Level::Info,
        2 => Level::Debug,
        3 | _ => Level::Trace,
    };
}

fn parse_from_args() -> Level {
    let mut verbose_level = 1;
    for argument in env::args().skip(1) {
        if argument == "--verbose" {
            verbose_level = verbose_level + 1;
        }

        if argument == "-v" {
            verbose_level = verbose_level + 1;
        }

        if argument.starts_with("--verbose=") {
            if let Some(count) = argument.get(("--verbose=".len())..) {
                if let Ok(value) = count.parse() {
                    verbose_level = value;
                }
            }
        }

        if argument.starts_with("-v=") {
            if let Some(count) = argument.get(("-v=".len())..) {
                if let Ok(value) = count.parse() {
                    verbose_level = value;
                }
            }
        }

        if !argument.starts_with("-") {
            break;
        }
    }

    return log_level(verbose_level);
}

fn parse_from_env() -> Level {
    if let Ok(inherited_log_level) = env::var("ETRAIN_LOG_LEVEL") {

        return if Level::Critical.as_str() == inherited_log_level {
            Level::Critical
        } else if Level::Error.as_str() == inherited_log_level {
            Level::Error
        } else if Level::Warning.as_str() == inherited_log_level {
            Level::Warning
        } else if Level::Info.as_str() == inherited_log_level {
            Level::Info
        } else if Level::Debug.as_str() == inherited_log_level {
            Level::Debug
        } else if Level::Trace.as_str() == inherited_log_level {
            Level::Trace
        } else {
            if let Ok(value) = inherited_log_level.parse() {
                log_level(value)
            } else {
                Level::Info
            }
        };
    }

    return Level::Info;
}
