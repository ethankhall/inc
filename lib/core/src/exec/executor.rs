use core::BASE_APPLICATION_NAME;
use core::cli::find_commands_avalible;
use docopt::Docopt;
use std::env::{self, current_exe, var};
use std::process::Command;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;
use std::fmt::Debug;

pub struct CliError {
    pub code: i32,
    pub message: String
}

impl CliError {
    pub fn new(code: i32, message: String) -> Self {
        CliError { code: code, message: message }
    }
}

pub type CliResult = Result<i32, CliError>;

pub type CliParseResults = Result<(), CliParseError>;

pub struct CliParseError { 
    pub fatal: bool,
    pub message: String
}

impl From<CliParseError> for CliError {
    fn from(err: CliParseError) -> CliError {
        CliError::new(101, err.message)
    }
}

pub fn call_main_without_stdin<'de, Flags: Debug + Deserialize<'de>>(
            exec: fn(Flags) -> CliResult,
            usage: &str,
            args: &[String]) -> CliResult
{
    trace!("Arguments to be passed into sub-command: {:?}", args);
    let docopt = Docopt::new(usage).unwrap()
        .options_first(true)
        .argv(args.iter().map(|s| &s[..]))
        .help(true);

    let flags = docopt.deserialize().map_err(|e| {
        CliParseError { fatal: e.fatal(), message: e.to_string() }
    })?;

    trace!("CLI Flags: {:?}", flags);

    exec(flags)
}

pub fn execute_external_command(cmd: &PathBuf, args: &[String]) -> CliResult {
    let command_exe = format!("{}-{:?}{}", BASE_APPLICATION_NAME, cmd, env::consts::EXE_SUFFIX);

    let path = find_commands_avalible().get(&command_exe).map(|x| x.clone().binary.path);
    let command = match path {
        Some(command) => command,
        None => {
            return Err(CliError { code: 9, message: format!("Unable to find {:?}", cmd) })
        }
    };

    let mut command = Command::new(command);
    let swawn = command
        .args(args)
        .envs(build_env_updates())
        .spawn();

    if let Err(value) = swawn {
        return Err(CliError { code: 10, message: format!("Unable to execute command: {}", value) });
    }

    let output = swawn.unwrap().wait();

    return match output {
        Ok(code) => Ok(code.code().unwrap_or_else(|| 0)),
        Err(value) => Err(CliError { code: 10, message: format!("Unable to run {:?} it returned {}", args, value) }),
    };
}

pub fn execute_external_command_for_output(cmd: &PathBuf, args: &[String]) -> Result<String, CliError> {
    let command_exe = format!("{}-{:?}{}", BASE_APPLICATION_NAME, cmd, env::consts::EXE_SUFFIX);

    let path = find_commands_avalible().get(&command_exe).map(|x| x.clone().binary.path);
    let command = match path {
        Some(command) => command,
        None => {
            return Err(CliError { code: 11, message: format!("Unable to find {:?}", cmd) })
        }
    };

    let mut command = Command::new(command);
    let output = command
        .args(args)
        .envs(build_env_updates())
        .output();

    if let Err(value) = output {
        return Err(CliError { code: 12, message: format!("Unable to execute command: {}", value) })
    }

    let output = output.unwrap();

    if !output.status.success() {
        for line in String::from_utf8_lossy(&output.stdout).to_string().lines() {
            error!("OUT: {}", line);
        }
        for line in String::from_utf8_lossy(&output.stderr).to_string().lines() {
            error!("ERR: {}", line);
        }
        return Err(CliError { code: 12, message: format!(
            "Unable to run {:?} it returned {}",
            args,
            output.status
        ) });
    }

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
}

fn build_env_updates() -> HashMap<String, String> {
    let mut results: HashMap<String, String> = HashMap::new();
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