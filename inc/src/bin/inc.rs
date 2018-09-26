#[macro_use]
extern crate clap;
#[macro_use]
extern crate inc_lib;
#[macro_use]
extern crate log;
extern crate inc_commands;

use inc_lib::core::config::ConfigContainer;
use inc_lib::core::command::AvaliableCommands;
use inc_lib::core::logging::configure_logging;
use inc_lib::exec::executor::{execute_external_command, CliError};
use std::process;
use clap::{App, AppSettings, Arg, ArgGroup};
use inc_lib::core::BASE_APPLICATION_NAME;
use std::collections::HashMap;
use std::string::String;

use inc_commands::checkout;
use inc_commands::exec;
use inc_commands::list;

fn main() {
    let matches = App::new("inc")
        .version(crate_version!())
        .settings(&[
            AppSettings::AllowExternalSubcommands,
            AppSettings::VersionlessSubcommands,
            AppSettings::ArgRequiredElseHelp,
        ])
        .global_setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("warn")
                .long("warn")
                .short("w")
                .help("Only display warning messages")
                .global(true),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .multiple(true)
                .help("Increasing verbosity")
                .global(true),
        )
        .arg(
            Arg::with_name("quite")
                .long("quite")
                .short("q")
                .help("Only error output will be displayed")
                .global(true),
        )
        .group(ArgGroup::with_name("logging").args(&["verbose", "quite", "warn"]))
        .subcommand(checkout::subcommand())
        .subcommand(exec::subcommand())
        .subcommand(list::subcommand())
        .get_matches_safe();

    let matches = match matches {
        Ok(v) => v,
        Err(err) => err.exit(),
    };

    configure_logging(
        matches.occurrences_of("verbose") as i32,
        matches.is_present("warn"),
        matches.is_present("quite"),
    );

    let avaliable_commands = AvaliableCommands::new();
    let config_container = match ConfigContainer::new() {
        Ok(value) => value,
        Err(s) => {
            error!("{}", s);
            process::exit(2);
        }
    };

    let result = match matches.subcommand() {
        ("checkout", Some(sub_m)) => checkout::execute(sub_m, avaliable_commands, config_container),
        ("exec", Some(sub_m)) => exec::execute(sub_m, avaliable_commands, config_container),
        ("list", Some(sub_m)) => list::execute(sub_m, avaliable_commands, config_container),
        (external, Some(sub_m)) => match avaliable_commands
            .find_command(format!("{}-{}", BASE_APPLICATION_NAME, external))
        {
            None => Err(CliError::new(
                2,
                format!(
                    "{} is not reconized. Please use `--list-commands` to see avaliable commands.",
                    external
                ),
            )),
            Some(cmd) => {
                let values: Vec<String> = match sub_m.values_of("") {
                    Some(v) => v.map(|x| s!(x)).collect(),
                    None => Vec::new(),
                };
                execute_external_command(&cmd.binary().path, &values, HashMap::new())
            }
        },
        e @ _ => {
            trace!("Something strange happened here... {:?}", e);
            Err(CliError::new(-1, s!("Command Unknown...")))
        }
    };

    let return_code = match result {
        Ok(value) => value,
        Err(err) => {
            error!("{}", err.message);
            err.code
        }
    };

    process::exit(return_code);
}
