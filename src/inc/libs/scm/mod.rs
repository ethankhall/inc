pub mod services;
pub mod provider;
pub mod api;
pub(crate) mod util;

use std::path::Path;

pub const DEFAULT_CHECKOUT_SOURCE: &'static str = "github";
pub const PRE_DEFINED_CHECKOUT_SOURCES: &'static [&'static str] = &["github"];

pub type ScmUrl = String;

pub(crate) trait ScmProvier {
    fn sugested_checkout_name(&self, url: &ScmUrl) -> Option<String>;
    fn do_checkout(&self, url: &ScmUrl, destination: &Path) -> Result<i32, CheckoutError>;
    fn handles_url(&self, url: &ScmUrl) -> bool;
}

#[derive(Debug)]
pub struct CheckoutError {
    pub error: String,
}

pub trait ScmService {
    fn generate_url(&self, user_input: String) -> Result<ScmUrl, CheckoutError>;
    fn name(&self) -> String;
}
