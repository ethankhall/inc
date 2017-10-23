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
            args: &Vec<String>) -> CliResult
{
    trace!("Arguments to be passed into sub-command: {:?}", args);
    let docopt = Docopt::new(usage).unwrap()
        .argv(args.clone())
        .options_first(true)
        .help(false);

    // trace!("Options: {:?}", docopt);

    let flags = docopt.deserialize().map_err(|e| {
        CliParseError { fatal: e.fatal(), message: e.to_string() }
    })?;

    trace!("CLI Flags: {:?}", flags);

    exec(flags)
}

pub fn execute_external_command(cmd: &PathBuf, args: &[String]) -> CliResult {
    let command_exe = format!("{:?}{}", cmd.to_str().unwrap(), env::consts::EXE_SUFFIX);
    let mut command = build_command(command_exe, args);

    let swawn = command
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

pub fn execute_external_command_for_output(cmd: &PathBuf, args: &[String], env: &HashMap<&str, &str>) -> Result<String, CliError> {
    let command_exe = format!("{}{}", cmd.to_str().unwrap(), env::consts::EXE_SUFFIX);
    let mut command = build_command(command_exe, args);
    command.envs(env.into_iter());
    
    let output = command.output();

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

    return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
}

fn build_command(cmd: String, args: &[String]) -> Box<Command> {
    let mut command_string = String::new();
    command_string.push_str(cmd.as_str());
    for arg in args.iter() {
        command_string.push_str(" ");
        command_string.push_str(arg.as_str());
    }

    let mut command = build_cmd_for_platform();
    command
        .arg(command_string)
        .envs(build_env_updates());

    return Box::from(command);
}

fn build_cmd_for_platform() -> Command {
    if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C");
        return cmd;
    } else {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        return cmd;
    }
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