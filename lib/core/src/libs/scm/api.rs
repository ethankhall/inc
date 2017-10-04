use slog::{Logger, Level};
use libs::scm::{ScmUrl, CheckoutError, ScmProvier};
use libs::scm::util::compute_destination;
use libs::scm::provider::git::{GitScm};
use libs::process::SystemBinary;
use libs::scm::services::build_service_map;

pub fn build_url_from_service(logger: &Logger, level: Level, service: String, user_input: String, command: &Vec<SystemBinary>) -> Result<ScmUrl, CheckoutError> {
    slog_debug!(logger, "Origonal input: {}", service);
    let service_map = build_service_map(logger, level, command);
    let service = service.to_lowercase();
    let service = service.as_str();

    return match service_map.get(service) {
        Some(svc) => svc.generate_url(user_input),
        None => Err(CheckoutError { error: format!("Unable to find determine how to execute {}", service)})
    }
}

pub fn checkout(logger: &Logger, repo_url: ScmUrl, destination: Option<String>) -> Result<i32, CheckoutError> {
    let git_provider = GitScm { logger: logger };
    let providers: Vec<&ScmProvier> = vec![ &git_provider ];

    let scm_provider = providers.into_iter().find(|x| x.handles_url(&repo_url));
    if scm_provider.is_none() {
        return Err(CheckoutError { error: format!("Unable to find scm for {}", repo_url) });
    }
    let scm_provider = scm_provider.unwrap();

    let suggested_name = scm_provider.sugested_checkout_name(&repo_url);
    let destination = compute_destination(logger, destination, suggested_name);

    return scm_provider.do_checkout(&repo_url, destination.as_path());
}