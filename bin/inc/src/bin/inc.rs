#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate inc_core;
extern crate docopt;

use inc_core::core::logging::configure_logging;
use inc_core::exec::executor::{CliResult, call_main_without_stdin};
use std::process;
use docopt::Docopt;

macro_rules! each_subcommand{
    ($mac:ident) => {
        $mac!(checkout);
        $mac!(exec);
        $mac!(help);
    }
}

macro_rules! declare_mod {
    ($name:ident) => ( pub mod $name; )
}

each_subcommand!(declare_mod);

const USAGE: &'static str = "Inc[luding] your configuration, one step at a time.

Usage:
    inc [options] <command> [--] [<args>...]
    inc --list
    inc --version
    inc --help

Options:
    -h, --help                  Show this screen.
    -v, --verbose ...           Increasing verbosity.
    -w, --warn                  Only display warning messages.
    -q, --quiet                 No output printed to stdout.
    --version                   Output the version of the command
    --list                      List all commands inc supports.
  
Some common inc commands are (see all commands with --list):
    help                        Prints this output
    exec                        Runs a command inside the project.
    checkout                    Checks out a project from an SCM.";

#[derive(Debug, Deserialize)]
pub(crate) struct Options {
    pub arg_command: String,
    pub arg_args: Option<Vec<String>>,
    pub flag_version: bool,
    pub flag_help: bool,
    pub flag_verbose: i32,
    pub flag_quiet: bool,
    pub flag_warn: bool,
    pub flag_list: bool,
}

fn main(){
    let docopt = Docopt::new(USAGE).unwrap()
        .options_first(true)
        .help(false);

    let options: Options = docopt.deserialize().map_err(|e| {
        println!("fatal: {}, message: {}", e.fatal(), e.to_string());
        e.exit();
    }).unwrap();
    
    configure_logging(options.flag_verbose, options.flag_warn, options.flag_quiet);

    if options.flag_help {
        info!("{}", USAGE);
        process::exit(0);
    }

    if options.flag_list {
        info!("I don't know how to do that.... Yet!");
        process::exit(0);
    }

    let mut args: Vec<String> = Vec::new();
    args.push(options.arg_command);
    &options.arg_args.unwrap_or_else(|| vec![]).iter().for_each(|x| {
        args.push(x.clone());
    });

    let result = try_execute_builtin_command(&args);
    if result.is_some() {
        let exit_code = match result.unwrap() {
            Ok(value) => value,
            Err(value) => {
                error!("Error executing sub command: {:?}", value.message);
                102
            }
        };
        process::exit(exit_code);
    };

    process::exit(1);
}

fn try_execute_builtin_command(args: &Vec<String>) -> Option<CliResult> {
    let command = args[0].clone();
    macro_rules! cmd {
        ($name:ident) => (if command == stringify!($name).replace("_", "-") {
            let r = call_main_without_stdin($name::execute,
                                                   $name::USAGE,
                                                   &args);
            return Some(r);
        })
    }
    each_subcommand!(cmd);

    None
}