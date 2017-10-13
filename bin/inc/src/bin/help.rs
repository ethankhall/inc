use inc_core::exec::executor::{CliResult};
use docopt::ArgvMap;

pub const USAGE: &'static str = "Help! Help! I need an adult!!!

Usage:
    help
    help --version";

pub(crate) fn execute(_options: ArgvMap) -> CliResult {
    info!("Help! Help! I need an adult!");
    return Ok(1);
}