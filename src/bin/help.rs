use inc::exec::executor::{CliResult};

#[derive(Deserialize, Debug)]
pub(crate) struct Options {
}

pub const USAGE: &'static str = "Help! Help! I need an adult!!!

Usage:
    help
    help --version";

pub(crate) fn execute(_options: Options) -> CliResult {
    info!("Help! Help! I need an adult!");
    return Ok(1);
}