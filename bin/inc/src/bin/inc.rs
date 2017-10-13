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
    let doc_opts:Options = Docopt::new(USAGE)
        .and_then(|d| d.options_first(true)
        .help(false)
        .deserialize())
        .unwrap_or_else(|e| e.exit());
    
    configure_logging(doc_opts.flag_verbose, doc_opts.flag_warn, doc_opts.flag_quiet);

    if doc_opts.flag_help {
        info!("{}", USAGE);
        process::exit(0);
    }

    let mut args: Vec<String> = Vec::new();
    args.push(doc_opts.arg_command);
    &doc_opts.arg_args.unwrap_or_else(|| vec![]).iter().for_each(|x| {
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