use libs::scm::{ScmUrl, ScmService, CheckoutError};
use slog::{Logger, Level};
use libs::process::{SubProcessArguments, run_command_with_output};
use std::collections::HashMap;

pub fn build_service_map<'a> (sub_commands: Vec<String>) -> HashMap<String, &'a ScmService> {
    let mut result: HashMap<String, &ScmService>= HashMap::new();
    result.insert(String::from("github"), &GitHubScmService {} );

    // sub_commands.into_iter()
    return result;
}

struct GitHubScmService {
}

impl ScmService for GitHubScmService {
    fn generate_url(&self, user_input: String) -> Result<ScmUrl, CheckoutError> {
        return Ok(format!("git@github.com:{0}.git", user_input));
    }

    fn name(&self) -> String {
        return String::from("github");
    }
}

struct ExternalScmService<'a> {
    pub logger: &'a Logger,
    pub log_level: Level,
    pub command_name: String,
    pub service_name: String
}

impl<'a>  ScmService for ExternalScmService<'a> {

    fn generate_url(&self, user_input: String) -> Result<ScmUrl, CheckoutError> {

        let args = SubProcessArguments { command: self.command_name.clone(), arguments: vec![user_input] };
        let stdout = run_command_with_output(self.logger, self.log_level, args);

        return match stdout {
            Ok(value) => Ok(value),
            Err(value) => Err(CheckoutError { error: value })
        }
    }

    fn name(&self) -> String {
        return self.service_name.clone();
    }
}