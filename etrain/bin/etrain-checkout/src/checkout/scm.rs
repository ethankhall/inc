use slog::Logger;
use std::path::Path;
use git::get_git_checkout;

#[derive(Debug)]
pub struct CheckoutError {
    pub error: String,
}

pub type ScmUrl = String;

pub fn create_url(logger: Logger, service: &str, repo: &str) -> Result<String, CheckoutError> {
    let service = service.to_lowercase();
    return if service == "github" {
        Ok(format!("git@github.com:{0}.git", repo))
    } else {
        Err(CheckoutError { error: String::from("Unknown service!") })
    };
}

pub fn do_scm_checkout(
    logger: Logger,
    url: String,
    destination: Option<&str>,
) -> Result<i32, CheckoutError> {
    slog_trace!(logger, "URL to clone: {}", url);
    if let Some(git_checkout) = get_git_checkout(logger, url) {
        return git_checkout.do_checkout(Path::new("/"));
    }

    return Ok(1);
}
