#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate yaml_rust;

pub mod logging;
pub mod cli;
pub mod config;
pub mod command;
pub mod mains;

pub const BASE_APPLICATION_NAME: &'static str = "etrain";