pub(crate) mod util;
pub(crate) mod git;
pub mod api;

#[cfg(test)]
pub(crate) mod git_test;

use std::path::Path;

pub const DEFAULT_CHECKOUT_SOURCE: &'static str = "github";
pub const PRE_DEFINED_CHECKOUT_SOURCES: &'static [&'static str] = &["github"];

#[derive(Debug)]
pub struct CheckoutError {
    pub error: String,
}

pub type ScmUrl = String;

pub(crate) trait ScmProvier {
    fn sugested_checkout_name(&self, url: &ScmUrl) -> Option<String>;
    fn do_checkout(&self, url: &ScmUrl, destination: &Path) -> Result<i32, CheckoutError>;
    fn handles_url(&self, url: &ScmUrl) -> bool;
}
