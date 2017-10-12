pub mod cli;
pub mod config;
pub mod command;

#[cfg(test)]
pub(crate) mod config_test;

pub const BASE_APPLICATION_NAME: &'static str = "inc";