use inc::core::config::{ConfigContainer, ExecConfig};
use inc::exec::executor::{execute_external_command, CliResult};
use std::path::PathBuf;
use std::fmt::Write;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use inc::core::command::AvaliableCommands;

pub(crate) fn subcommand<'a, 'b>() -> App<'a, 'b> {
    return SubCommand::with_name("exec")
        .about("Execute commands from the project.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("list-commands")
                .long("list-commands")
                .help("List all of the avaliable commands."),
        )
        .arg(
            Arg::with_name("command")
                .help("Name of the command to execute.")
                .takes_value(true)
                .required(true)
                .required_unless("list-commands"),
        );
}

pub(crate) fn execute(
    args: &ArgMatches,
    _commands: AvaliableCommands,
    config: ConfigContainer,
) -> CliResult {
    let exec_configs = config.get_exec_configs();

    if args.is_present("list-commands") {
        info!("{}", generate_list_options(&exec_configs));
        return Ok(0);
    }

    let config = exec_configs
        .commands
        .get(&s!(args.value_of("command").unwrap()))
        .unwrap();
    for command_entry in config.clone().commands.into_iter() {
        if config.clone().commands.len() > 1 {
            info!("** Executing `{}`", command_entry);
        }
        let mut command_list: Vec<String> =
            command_entry.split(" ").map(|x| String::from(x)).collect();
        let command_exec = command_list.remove(0);

        debug!("Executing {:?} {:?}", command_exec, command_list);
        let result = execute_external_command(&PathBuf::from(command_exec.clone()), &command_list);
        match result {
            Ok(value) => {
                if value != 0 {
                    error!("Command: `{}` returned {}", command_exec, value);
                    return Ok(value);
                }
            }
            Err(_err) => {
                error!("Error while executing {:?}!", command_exec);
                return Ok(17);
            }
        }
    }
    return Ok(0);
}

fn generate_list_options(config: &ExecConfig) -> String {
    let mut list = String::new();
    write!(&mut list, "Avaliable Commands:\n").unwrap();

    let command_map = config.clone().commands;
    let mut commands: Vec<&String> = command_map.keys().collect();
    commands.sort();

    for key in commands.iter() {
        let value = command_map.get(*key).unwrap();
        write!(&mut list, " - name: {}\n", key).unwrap();
        write!(&mut list, "   description: {}\n", value.description).unwrap();
        write!(&mut list, "   commands:\n").unwrap();
        for command in value.commands.iter() {
            write!(&mut list, "     - {}\n", command).unwrap();
        }
    }
    return list;
}
