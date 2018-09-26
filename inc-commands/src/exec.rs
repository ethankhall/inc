use inc_lib::core::config::{ConfigContainer, ExecConfig, CommandAndEnv};
use inc_lib::exec::executor::{execute_external_command, CliResult, CliError};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fmt::Write;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use inc_lib::core::command::AvaliableCommands;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
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

pub fn execute(
    args: &ArgMatches,
    _commands: AvaliableCommands,
    config: ConfigContainer,
) -> CliResult {
    let exec_configs = config.get_exec_configs();

    if args.is_present("list-commands") {
        info!("{}", generate_list_options(&exec_configs));
        return Ok(0);
    }

    let command_to_exec = args.value_of("command").unwrap();
    debug!("Going to exec {}", command_to_exec);

    let config = match exec_configs.commands.get(command_to_exec) {
        Some(value) => value,
        None => {
            return Err(CliError::new(2, format!("Unable to find command list for {}! Failing!", command_to_exec)));
        }
    };

    let command_defined_in = exec_configs.command_defintions.get(command_to_exec);

    let commands: Vec<CommandAndEnv> = config.clone().commands.into_iter().map(|x| x.to_command_and_envs()).collect();
    let command_count = commands.len();

    for command_entry in commands.into_iter() {
        if command_count > 1 {
            info!("** Executing `{}`", command_entry.command);
        }

        let mut command_list: Vec<String> =
            command_entry.command.split(" ").map(|x| String::from(x)).collect();
        let command_exec = command_list.remove(0);

        let mut extra_env: HashMap<String, String> = HashMap::new();
        
        for (key, value) in command_entry.command_env {
            extra_env.insert(key, value);
        }
        if let Some(path) = command_defined_in {
            extra_env.insert(s!("INC_PROJECT_DIR"), s!(path.parent().unwrap().to_str().unwrap()));
        }

        debug!("Executing {:?} {:?} defined in {:?}", command_exec, command_list, command_defined_in);
        let result = execute_external_command(&PathBuf::from(command_exec.clone()), &command_list, extra_env);
        match result {
            Ok(value) => {
                if value != 0 {
                    error!("Command: `{}` returned {}", command_entry.command, value);
                    return Ok(value);
                }
            }
            Err(_err) => {
                error!("Error while executing `{:?}`!", command_entry.command);
                return Ok(17);
            }
        }
    }
    return Ok(0);
}

fn generate_list_options(config: &ExecConfig) -> String {
    let mut list = String::new();
    write!(&mut list, "Avaliable Commands:\n").unwrap();

    let command_map = config.commands.clone();
    let mut commands: Vec<&String> = command_map.keys().collect();
    commands.sort();

    for key in commands.iter() {
        let value = command_map.get(*key).unwrap().clone();
        write!(&mut list, " - name: {}\n", key).unwrap();
        write!(&mut list, "   description: {}\n", value.description).unwrap();
        write!(&mut list, "   commands:\n").unwrap();
        let command_list: Vec<CommandAndEnv> = value.commands.into_iter().map(|x| x.to_command_and_envs()).collect();
        for command in command_list {
            write!(&mut list, "     - command: {}\n", command.command).unwrap();
            write!(&mut list, "       env: {:?}\n", command.command_env).unwrap();
        }
    }
    return list;
}
