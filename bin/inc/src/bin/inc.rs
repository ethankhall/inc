extern crate inc_impl;

use inc_impl::commands::build_main_command;
use inc_impl::core::mains::root_main;
use std::process;

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

fn do_main() -> i32 {
    let command = build_main_command();
    return root_main(&command);
}