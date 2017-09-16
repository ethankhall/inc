use git::GitCheckout;
use slog::Logger;


#[derive(Debug)]
pub struct CheckoutError {
    error: &'static str,
}

pub fn createScmCheckout(
    logger: Logger,
    service: String,
    repo: String,
    destination: String,
) -> Result<i32, CheckoutError> {
    let service = service.to_lowercase();
    return if service == "github" {
        let git_checkout =
            GitCheckout::new(logger, format!("git@github.com:{0}.git", repo), destination);
        git_checkout.do_checkout()
    } else {
        Err(CheckoutError { error: "Unknown service!" })
    };
}
