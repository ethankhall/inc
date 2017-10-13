use inc_core::core::config::ConfigContainer;
use inc_core::exec::executor::{CliResult, execute_external_command};
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub(crate) struct Options {
    arg_command: Option<String>,
    flag_help: bool,
    flag_verbose: Option<String>,
    flag_list: bool,
}

pub const USAGE: &'static str = "Execute commands from the project.

Usage:
    inc-exec [options] <command>
    inc-exec (-h | --help)
    inc-exec --list

Options:
    -h, --help          Display this message.
    -v, --verbose ...   Increasing verbosity.
    -q, --quiet         No output printed to stdout
    --list              List all of the avaliable commands.";

pub(crate) fn execute(options: Options) -> CliResult {
    trace!("Options to exec: {:?}", options);
    if options.flag_help {
        info!("{}", USAGE);
        return Ok(0);
    }

    let exec_configs = ConfigContainer::new().get_exec_configs();

    if options.flag_help {
        info!("{}", USAGE);
        return Ok(0);
    }

    let command = match options.arg_command {
        Some(command) => command,
        None => {
            error!("Option or command must be passed! Run inc exec --help for options");
            return Ok(1);
        }
    };
    
    println!("command: {}", command);
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
            Err(err) => {
                error!("Error while executing {:?}!", command_exec);
                return Err(err);
            }
        }
    }
    return Ok(0);
}