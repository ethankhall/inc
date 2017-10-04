use slog::Logger;
use libs::scm::{ScmUrl, CheckoutError, ScmProvier};
use libs::scm::util::compute_destination;
use libs::scm::provider::git::{GitScm};
use libs::process::SystemBinary;

pub fn build_url_from_service(logger: &Logger, service: String, repo: String, command: Vec<SystemBinary>) -> Result<String, CheckoutError> {
    slog_debug!(logger, "Origonal input: {}", service);
    let service = service.to_lowercase();
    return if service == "github" {
        Ok(format!("git@github.com:{0}.git", repo))
    } else {
        Err(CheckoutError { error: String::from("Unknown service!") })
    };
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