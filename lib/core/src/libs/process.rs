use std::env::{current_exe, var};
use std::vec::Vec;
use slog::{Logger, Level};
use std::process::Command;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemCommand {
    pub binary: SystemBinary,
    pub alias: String,
    pub sub_commands: Vec<SystemBinary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemBinary {
    pub path: PathBuf,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct SubProcessArguments {
    pub command: PathBuf,
    pub arguments: Vec<String>,
}

pub fn run_command_with_output(
    logger: &Logger,
    log_level: Level,
    args: SubProcessArguments,
) -> Result<String, String> {
    slog_trace!(logger, "Trying to execute {:?}", args.command);


    let mut command = Command::new(args.command.clone());
    let output = command
        .args(args.arguments)
        .env("INC_LOG_LEVEL", log_level.as_str())
        .env("PATH", build_path())
        .output()
        .expect("command failed to start");

    if !output.status.success() {
        for line in String::from_utf8_lossy(&output.stdout).to_string().lines() {
            slog_error!(logger, "OUT: {}", line);
        }
        for line in String::from_utf8_lossy(&output.stderr).to_string().lines() {
            slog_error!(logger, "ERR: {}", line);
        }
        return Err(format!(
            "Unable to run {:?} it returned {}",
            args.command,
            output.status
        ));
    }

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
}


fn build_path() -> String {
    let path_extension = if let Ok(path) = current_exe() {
        let mut path = path.canonicalize().unwrap();
        path.pop();
        format!(":{}", path.as_os_str().to_str().unwrap())
    } else {
        String::new()
    };

    let path = var("PATH").unwrap();
    return format!("{}{}", path, path_extension);
}
