#[macro_use]
extern crate slog;
extern crate clap;
extern crate etrain_core;
extern crate checkout;

use etrain_core::logging::{logging, get_verbosity_level};
use clap::{App, Arg};
use checkout::scm::createScmCheckout;
use std::process;

fn main() {
    let exit_code = doMain();
    process::exit(exit_code);
}

fn doMain() -> i32 {
    let logger = logging(get_verbosity_level(), "etrain-checkout");

    let matches = App::new("etrain checkout")
        .about("Checkout a project")
        .arg(
            Arg::with_name("service")
                .short("s")
                .help("Where to checkout from. A lot of cases will be github")
                .default_value("github")
                .possible_value("github"),
        )
        .arg(
            Arg::with_name("SOURCE")
                .help("What to checkout")
                .required(true)
                .multiple(true),
        )
        .arg(Arg::with_name("destination").short("d").help(
            "Directory to checkout into",
        ))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .default_value("1")
                .help("Sets the level of verbosity."),
        )
        .get_matches();

    let service = matches.value_of("service").unwrap_or_default();
    let source = matches.value_of("SOURCE").unwrap();

    slog_debug!(logger, "Checking out {} from {}", source, service);

    let result = createScmCheckout(
        logger.clone(),
        String::from(service),
        String::from(source),
        String::from("1"),
    );
    slog_trace!(logger, "Results from checkout: {:?}", result);

    return match result {
        Ok(value) => value,
        _ => 1,
    };
}
