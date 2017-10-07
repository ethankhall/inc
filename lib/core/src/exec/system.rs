use exec::Execution;
use std::path::PathBuf;
use slog::{Level, Logger};
use std::env::{current_exe, var};
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SystemExecution {
    pub command: PathBuf,
    pub log_level: Level,
    pub logger: Logger,
}

impl Execution<i32> for SystemExecution {
    fn execute(&self, args: &Vec<String>) -> Result<i32, String> {
        let mut command = Command::new(self.command.clone());
        let swawn = command
            .args(args)
            .envs(build_env_updates(self.log_level))
            .spawn();

        if let Err(value) = swawn {
            return Err(format!("Unable to execute command: {}", value));
        }

        let output = swawn.unwrap().wait();

        return match output {
            Ok(code) => Ok(code.code().unwrap_or_else(|| 0)),
            Err(value) => Err(format!("Unable to run {:?} it returned {}", args, value)),
        };
    }
}

#[derive(Debug, Clone)]
pub struct OutputCapturingSystemExecution {
    pub command: PathBuf,
    pub log_level: Level,
    pub logger: Logger,
}

impl Execution<String> for OutputCapturingSystemExecution {
    fn execute(&self, args: &Vec<String>) -> Result<String, String> {
        let logger = &(self.logger);
        let mut command = Command::new(self.command.clone());
        let output = command
            .args(args)
            .envs(build_env_updates(self.log_level))
            .output();

        if let Err(value) = output {
            return Err(format!("Unable to execute command: {}", value));
        }

        let output = output.unwrap();

        if !output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).to_string().lines() {
                slog_error!(logger, "OUT: {}", line);
            }
            for line in String::from_utf8_lossy(&output.stderr).to_string().lines() {
                slog_error!(logger, "ERR: {}", line);
            }
            return Err(format!(
                "Unable to run {:?} it returned {}",
                args,
                output.status
            ));
        }

        return Ok(String::from_utf8_lossy(&output.stdout).to_string());
    }
}

fn build_env_updates(log_level: Level) -> HashMap<String, String> {
    let mut results: HashMap<String, String> = HashMap::new();
    results.insert(
        String::from("INC_LOG_LEVEL"),
        String::from(log_level.as_str()),
    );
    results.insert(String::from("PATH"), build_path());

    return results;
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
