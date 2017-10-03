#[macro_use]
extern crate slog;

extern crate etrain_core;

use etrain_core::logging::{logging, get_verbosity_level};
use etrain_core::config::{ConfigParser, ConfigContainer, ConfigSource, ConfigValue};
use etrain_checkout::command::

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

fn do_main() -> i32 {
    let prefix_string = format!("{}-checkout", BASE_APPLICATION_NAME);
    let logger = logging(get_verbosity_level(), prefix_string.clone());
    
    let cli_resolver = CliResolver { logger: logger.clone(), prefix: prefix_string};
    let commands = cli_resolver.find_commands();

    let command = CheckoutCommand{};
    
}
