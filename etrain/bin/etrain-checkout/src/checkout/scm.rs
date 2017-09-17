use git::GitCheckout;
use slog::Logger;


#[derive(Debug)]
pub struct CheckoutError {
    pub error: &'static str,
}

pub fn create_scm_checkout(logger: Logger, service: &str, 
        repo: &str, destination: Option<&str>) -> Result<i32, CheckoutError> {
    let service = service.to_lowercase();
    return if service == "github" {
        let git_checkout = GitCheckout::new(logger, format!("git@github.com:{0}.git", repo), destination);
        git_checkout.do_checkout()
    } else {
        Err(CheckoutError { error: "Unknown service!" })
    };
}
