use std::io::{stdout, stderr};
use chrono::Local;

use fern::Dispatch;
use log::{LogLevelFilter, LogLevel};

pub fn configure_logging(verbose: i32, warn: bool, quite: bool) {
    let level = if quite {
        0
    } else if warn {
        1
    } else {
        verbose + 2
    };

    let level = log_level(level);
    let dispatch = Dispatch::new();
    
    let result = configure_logging_output(level, dispatch)
    .level(level)
    .chain(
        Dispatch::new()
            .filter(|log_meta| LogLevel::Warn <= log_meta.level())
            .chain(stdout()))
    .chain(
        Dispatch::new()
            .filter(|log_meta| LogLevel::Error == log_meta.level())
            .chain(stderr()))
    .apply();

    if result.is_err() {
        panic!("Logger already initialized...");
    }
}

fn log_level(number_of_verbose: i32) -> LogLevelFilter {
    return match number_of_verbose {
        0 => LogLevelFilter::Error,
        1 => LogLevelFilter::Warn,
        2 => LogLevelFilter::Info,
        3 => LogLevelFilter::Debug,
        4 | _ => LogLevelFilter::Trace,
    };
}

fn configure_logging_output(logging_level: LogLevelFilter, dispatch: Dispatch) -> Dispatch {
    if logging_level == LogLevelFilter::Trace {
        return dispatch.format(|out, message, record| {
            out.finish(format_args!(
            "{}[{}][{}] {}",
            Local::now().format("[%Y-%m-%d - %H:%M:%S]"),
            record.target(),
            record.level(),
            message))
        });
    } if logging_level == LogLevelFilter::Debug {
        return dispatch.format(|out, message, record| {
            out.finish(format_args!(
            "{}[{}] {}",
            Local::now().format("[%Y-%m-%d - %H:%M:%S]"),
            record.level(),
            message))
        });
    } else { 
        return dispatch.format(|out, message, _record| {
            out.finish(format_args!("{}", message))
        });
    }
}