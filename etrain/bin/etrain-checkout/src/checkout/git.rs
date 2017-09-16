use slog::Logger;
use scm::CheckoutError;

#[derive(Debug)]
pub struct GitCheckout {
    logger: Logger,
    url: String,
    destination: String,
}

impl GitCheckout {
    pub fn new(logger: Logger, url: String, destination: String) -> GitCheckout {
        GitCheckout {
            logger,
            url,
            destination,
        }
    }

    pub fn do_checkout(&self) -> Result<i32, CheckoutError> {
        return Ok(1);
    }
}
