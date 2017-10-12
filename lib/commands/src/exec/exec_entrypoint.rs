use inc_core::core::command::MainCommand;
use inc_core::core::config::ConfigContainer;
use inc_core::core::BASE_APPLICATION_NAME;
use inc_core::exec::executor::Executor;
use inc_core::exec::Execution;
use docopt::Docopt;
use inc_core::exec::system::SystemExecution;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct Args {
    arg_command: String,
    arg_extras: Option<Vec<String>>,
    flag_version: bool,
    flag_help: bool,
    flag_verbose: Option<String>,
}

pub struct ExecCommand {
    config_container: ConfigContainer
}

impl ExecCommand {
    pub fn new(config_container: ConfigContainer) -> Self {
        ExecCommand { config_container: config_container }
    }

    fn build_usage(&self, commands: Vec<&String>) -> String {
        let command_list: Vec<String> = commands.iter().map(|x| format!("  {}", x)).collect();
        let command_list: String = command_list.join("\n");

        return format!(
"Usage:
    inc-exec [options] <command> [--] [<extras>...]
    inc-exec (-h | --help | --version)

Options:
    -v <level>, --verbose=<level>        Enable more verbose output [ default: 1 ]

Flags:
    -h, --help       Prints help information

Args:
    <command>        The command to execute in the project.
    <extras>         This option is only valid if one command is to be executed.

Commands:
    {command_list}",
            command_list = command_list);
    }

    fn do_execute(&self, args: &Vec<String>) -> Result<i32, String> {
        let exec_configs = self.config_container.get_exec_configs();

        let doc_opts: Args = Docopt::new(self.build_usage(exec_configs.commands.keys().collect()))
            .and_then(|d| d.argv(args.into_iter()).parse())
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        if !exec_configs.commands.contains_key(&doc_opts.arg_command) {
            error!("Command {} doesn't exist in your configuration.", doc_opts.arg_command);
            return Ok(10);
        }

        let executor = Executor::new();
        let config = exec_configs.commands.get(&doc_opts.arg_command).unwrap();
        for command_entry in config.clone().commands.into_iter() {

            let mut command_list: Vec<String> = command_entry.split(" ").map(|x| String::from(x)).collect();
            let command_exec = command_list.remove(0);
            let command = SystemExecution {
                command: PathBuf::from(&command_exec)
            };

            debug!("Executing {:?} {:?}", command_exec, command_list);

            let result = executor.execute(&command, &command_list);
            match result {
                Ok(value) => {
                    if value != 0 {
                        error!("Command: `{:?}` returned {}", command, value);
                        return Ok(value);
                    }
                },
                Err(err) => {
                    error!("Error while executing {:?}!", command);
                    return Err(err);
                }
            }
        }
        return Ok(0);
    }
}

impl Execution<i32> for ExecCommand {
    fn execute(&self, args: &Vec<String>) -> Result<i32, String> {
        return self.do_execute(args);
    }
}

impl MainCommand for ExecCommand {
    fn execute(&self, args: &Vec<String>) -> i32 {
        return self.do_execute(args).expect("Error while executing exec!");
    }

    fn get_command_name(&self) -> String {
        return String::from("exec");
    }

    fn get_command_prefix(&self) -> String {
        return format!("{}-{}", BASE_APPLICATION_NAME, self.get_command_name());
    }

    fn get_description(&self) -> String {
        return String::from("Execute commands");
    }
}