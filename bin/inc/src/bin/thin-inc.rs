extern crate inc_core;
extern crate inc_commands;

use inc_commands::build_main_command;
use inc_commands::mains::root_main;
use std::process;

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

fn do_main() -> i32 {
    return root_main(|_config, command| { Box::new(build_main_command(command)) });
}
