use libs::scm::{ScmUrl, ScmService, CheckoutError};
use libs::process::SystemBinary;
use exec::executor::execute_external_command_for_output;
use std::collections::HashMap;
use core::BASE_APPLICATION_NAME;

pub fn build_service_map(sub_commands: &Vec<SystemBinary>,) -> HashMap<String, Box<ScmService>> {
    let mut result: HashMap<String, Box<ScmService>> = HashMap::new();
    result.insert(String::from("github"), Box::new(GitHubScmService {}));

    let service_prefix = format!("{}-checkout-service-", BASE_APPLICATION_NAME);

    for external_source in sub_commands.into_iter() {
        trace!("external_source: {:?}", external_source);
        if external_source.name.starts_with(service_prefix.as_str()) {
            let service_name = String::from(&external_source.name[(service_prefix.len())..]);
            let service = ExternalScmService::new(
                external_source.clone(),
                service_name.clone(),
            );
            result.insert(service_name, Box::new(service));
        }
    }

    return result;
}

struct GitHubScmService {}

impl ScmService for GitHubScmService {
    fn generate_url(&self, user_input: String) -> Result<ScmUrl, CheckoutError> {
        return Ok(format!("git@github.com:{0}.git", user_input));
    }

    fn name(&self) -> String {
        return String::from("github");
    }
}

struct ExternalScmService {
    pub binary: SystemBinary,
    pub service_name: String,
}

impl ExternalScmService {
    fn new(
        binary: SystemBinary,
        service_name: String,
    ) -> ExternalScmService {
        ExternalScmService {
            binary: binary,
            service_name: service_name,
        }
    }
}

impl ScmService for ExternalScmService {
    fn generate_url(&self, user_input: String) -> Result<ScmUrl, CheckoutError> {
        let result = execute_external_command_for_output(&(self.binary.clone().path), &(vec![user_input]));

        return match result {
            Ok(expr) => Ok(expr),
            Err(value) => Err(CheckoutError { error: value.message }),
        };
    }

    fn name(&self) -> String {
        return self.service_name.clone();
    }
}
