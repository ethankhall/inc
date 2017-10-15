use inc_core::core::config::{ConfigContainer, ExecConfig};
use inc_core::exec::executor::{CliResult, execute_external_command};
use std::path::PathBuf;
use std::fmt::Write;

#[derive(Deserialize, Debug)]
pub(crate) struct Options {
    arg_command: Option<String>,
    flag_help: bool,
    flag_verbose: Option<String>,
    flag_list: bool,
    flag_quiet: bool,
    flag_warn: bool,
}

pub const USAGE: &'static str = "Execute commands from the project.

Usage:
    inc-exec <command>
    inc-exec [options]
    inc-exec [options] <command>
    inc-exec --help
    inc-exec --list

Options:
    -h, --help          Display this message.
    -v, --verbose ...   Increasing verbosity.
    -q, --quiet         No output printed to stdout
    -w, --warn          Only display WARN and above outputs.
    --list              List all of the avaliable commands.";

pub(crate) fn execute(options: Options) -> CliResult {
    trace!("Options to exec: {:?}", options);
    if options.flag_help {
        info!("{}", USAGE);
        return Ok(0);
    }

    let configs = ConfigContainer::new();
    let exec_configs = configs.get_exec_configs();

    if options.flag_list {
        info!("{}", generate_list_options(&exec_configs));
        return Ok(0);
    }

    let command = match options.arg_command {
        Some(command) => command,
        None => {
            error!("Option or command must be passed! Run inc exec --help for options.");
            return Ok(1);
        }
    };
    
    if !exec_configs.commands.contains_key(&command) {
        error!("Command {} doesn't exist in your configuration.", command);
        return Ok(10);
    }

    let config = exec_configs.commands.get(&command).unwrap();
    for command_entry in config.clone().commands.into_iter() {

        let mut command_list: Vec<String> = command_entry.split(" ").map(|x| String::from(x)).collect();
        let command_exec = command_list.remove(0);
        
        debug!("Executing {:?} {:?}", command_exec, command_list);
        let result = execute_external_command(&PathBuf::from(command_exec.clone()), &command_list);
        match result {
            Ok(value) => {
                if value != 0 {
                    error!("Command: `{:?}` returned {}", command_exec, value);
                    return Ok(value);
                }
            },
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