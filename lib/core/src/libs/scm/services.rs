use libs::scm::{ScmUrl, ScmService, CheckoutError};
use slog::{Logger, Level};
use libs::process::{SubProcessArguments, run_command_with_output, SystemBinary};
use std::collections::HashMap;
use core::BASE_APPLICATION_NAME;

pub fn build_service_map(logger: &Logger, log_level: Level, sub_commands: &Vec<SystemBinary>) -> HashMap<String, Box<ScmService>> {
    let mut result: HashMap<String, Box<ScmService>>= HashMap::new();
    result.insert(String::from("github"), Box::new(GitHubScmService {} ));

    let service_prefix = format!("{}-checkout-service-", BASE_APPLICATION_NAME);

    for external_source in sub_commands.into_iter() {
        if external_source.name.starts_with(service_prefix.as_str()) {
            let service_name = String::from(&external_source.name[(service_prefix.len())..]);
            let service = ExternalScmService::new(logger.clone(), log_level, external_source.clone(), service_name.clone());
            result.insert(service_name, Box::new(service));
        }
    }


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

struct ExternalScmService {
    pub logger: Logger,
    pub log_level: Level,
    pub binary: SystemBinary,
    pub service_name: String
}

impl ExternalScmService {
    fn new(logger: Logger, level: Level, binary: SystemBinary, service_name: String) -> ExternalScmService {
        ExternalScmService { logger: logger, 
                log_level: level, 
                binary: binary, 
                service_name: service_name }
    }
}

impl ScmService for ExternalScmService {

    fn generate_url(&self, user_input: String) -> Result<ScmUrl, CheckoutError> {

        let args = SubProcessArguments { command: self.binary.clone().path, arguments: vec![user_input] };
        let stdout = run_command_with_output(&(self.logger), self.log_level, args);

        return match stdout {
            Ok(value) => Ok(String::from(value.trim())),
            Err(value) => Err(CheckoutError { error: value })
        }
    }

    fn name(&self) -> String {
        return self.service_name.clone();
    }
}