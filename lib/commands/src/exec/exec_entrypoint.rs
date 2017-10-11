use inc_core::core::command::MainCommand;
use inc_core::core::config::ConfigContainer;
use inc_core::core::BASE_APPLICATION_NAME;
use inc_core::exec::Execution;

pub struct ExecCommand {
    config_container: ConfigContainer
}

impl ExecCommand {
    pub fn new(config_container: ConfigContainer) -> Self {
        ExecCommand { config_container: config_container }
    }

    fn build_usage(&self) -> String {
        let command_list = String::from("!! NO COMMANDS DEFINED !!");

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
}

impl Execution<i32> for ExecCommand {
    fn execute(&self, args: &Vec<String>) -> Result<i32, String> {
        return Ok(1);
    }
}

impl MainCommand for ExecCommand {
    fn execute(&self, args: &Vec<String>) -> i32 {
        return 1;
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