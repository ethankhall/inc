use std::env::{var, set_var};
use std::io::stdout;
use std::str::FromStr;
use std::fmt::{Display, self};
use chrono::Local;

use fern::Dispatch;
use log::LogLevelFilter;

pub const LOG_LEVEL_DEFINITION: &'static str = "INC_LOG_LEVEL";

pub fn parse_from_args(args: &Vec<String>) -> IncLogLevel {
    if let Ok(level) = var(LOG_LEVEL_DEFINITION) {
        return level.parse().unwrap_or_else(|_| IncLogLevel::Trace);
    }

    let mut verbose_level = 1;
    for argument in args.into_iter().skip(1) {
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

fn log_level(number_of_verbose: u64) -> IncLogLevel {
    return match number_of_verbose {
        0 => IncLogLevel::Warn,
        1 => IncLogLevel::Info,
        2 => IncLogLevel::Debug,
        3 | _ => IncLogLevel::Trace,
    };
}

pub fn configure_logging(logging_level: Option<IncLogLevel>) {
    let level = LogLevelFilter::from(parse_log_level(logging_level));
    let dispatch = Dispatch::new();
    
    let result = configure_logging_output(level, dispatch)
    .level(level)
    .chain(stdout())
    .apply();

    if result.is_err() {
        panic!("Logger already initialized...");
    }
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

fn parse_log_level(logging_level: Option<IncLogLevel>) -> IncLogLevel {
    if logging_level.is_some() {
        let level = logging_level.unwrap();
        set_var(LOG_LEVEL_DEFINITION, format!("{}", level));
        return level;
    }

    return var(LOG_LEVEL_DEFINITION).unwrap_or_else(|_| String::from("Trace"))
        .parse().unwrap_or_else(|_| IncLogLevel::Trace);
}

#[derive(Eq, PartialEq)]
pub enum IncLogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Display for IncLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            IncLogLevel::Error => write!(f, "Error"),
            IncLogLevel::Warn => write!(f, "Warn"),
            IncLogLevel::Info => write!(f, "Info"),
            IncLogLevel::Debug => write!(f, "Debug"),
            IncLogLevel::Trace => write!(f, "Trace"),
       }
    }
}

impl FromStr for IncLogLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "Error" => IncLogLevel::Error,
            "Warn" => IncLogLevel::Warn,
            "Info" => IncLogLevel::Info,
            "Debug" => IncLogLevel::Debug,
            "Trace" => IncLogLevel::Trace,
            _ => IncLogLevel::Trace,
        };

        return Ok(value);
    }
}

impl From<IncLogLevel> for LogLevelFilter {
    fn from(level: IncLogLevel) -> Self {
        match level {
            IncLogLevel::Error => LogLevelFilter::Error,
            IncLogLevel::Warn => LogLevelFilter::Warn,
            IncLogLevel::Info => LogLevelFilter::Info,
            IncLogLevel::Debug => LogLevelFilter::Debug,
            IncLogLevel::Trace => LogLevelFilter::Trace,
        }
    }
}