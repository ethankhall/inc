use inc_core::exec::Execution;
use inc_core::libs::process::SystemCommand;
use std::vec::Vec;

const HELP_COMMAND_MESSAGE: &'static str = "\
Usage:
  inc [options] <command> [--] [<args>...]
  inc --version
  inc --help

Options:
  -h --help                 Show this screen.
  -v <l>, --verbose=<l>     Enable more verbose output [default: 1]
  
Commands:
  help
{command_list}
";

#[derive(Deserialize, Debug)]
pub(crate) struct HelpArgs {
    pub arg_command: String,
    pub arg_args: Option<Vec<String>>,
    pub flag_version: bool,
    pub flag_help: bool,
    pub flag_verbose: Option<String>,
}

pub struct HelpCommand {
    commands: Vec<String>,
}

impl HelpCommand {
    pub fn new(commands: &Vec<&SystemCommand>) -> Self {
        let commands = commands.iter().map(|x| x.alias.clone()).collect();
        HelpCommand {
            commands: commands,
        }
    }

    pub fn build_help_message(&self) -> String {
        let command_list: Vec<String> = self.commands.iter().map(|x| format!("  {}", x)).collect();
        let command_list: String = String::from(command_list.join("\n"));
        return String::from(HELP_COMMAND_MESSAGE.replace(
            "{command_list}",
            command_list.as_str(),
        ));
    }
}

impl Execution<()> for HelpCommand {
    fn execute(&self, _args: &Vec<String>) -> Result<(), String> {

        let help_message = self.build_help_message();
        info!("{}", help_message);

        return Ok(());
    }
}
