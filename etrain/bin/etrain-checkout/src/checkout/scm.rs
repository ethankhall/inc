extern crate url;

use git::GitCheckout;
use slog::Logger;
use url::{Url, ParseError};

#[derive(Debug)]
pub struct CheckoutError {
    pub error: String,
}

pub fn create_url(logger: Logger, service: &str, repo: &str) -> Result<String, CheckoutError> {
    let service = service.to_lowercase();
    return if service == "github" {
        Ok(format!("git@github.com:{0}.git", repo))
    } else {
        Err(CheckoutError { error: String::from("Unknown service!") })
    };
}

pub fn do_scm_checkout(logger: Logger, url: String, destination: Option<&str>) -> Result<i32, CheckoutError> {
    slog_trace!(logger, "URL to clone: {}", url);
    return if url.starts_with("git://") || url.starts_with("git@") {
        GitCheckout::new(logger, url, destination).do_checkout()
    } else {
        Err(CheckoutError { error: String::from("Unknown url!") })
    }
}