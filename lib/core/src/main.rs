use command::MainCommand;
use BASE_APPLICATION_NAME;

pub fn sub_command_run<T: MainCommand>(args: Vec<String>, command: &T) {
    let prefix_string = format!("{}-{}", BASE_APPLICATION_NAME, command.name());
}