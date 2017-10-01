#[macro_use]
extern crate slog;
extern crate clap;
extern crate etrain_core;
extern crate checkout;

use etrain_core::logging::{logging, get_verbosity_level};
use etrain_core::config::{ConfigParser, ConfigContainer, ConfigSource, ConfigValue};
use clap::{App, Arg};
use checkout::scm::{do_scm_checkout, create_url};
use std::process;

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

const DEFAULT_CHECKOUT_SOURCE: &'static str = "github";

fn get_default_checkout_source() -> String {
    let default_source = String::from(DEFAULT_CHECKOUT_SOURCE);
    let config_parser: ConfigContainer = ConfigParser::new();
    let default_checkout = config_parser.get_from_source(String::from("checkout.default"), ConfigSource::Home);
    let default_checkout_source = default_checkout.unwrap_or_else(|_| { return ConfigValue::String(default_source.clone())});

    return match default_checkout_source {
        ConfigValue::String(value) => value,
        _ => default_source.clone()
    };
}

fn do_main() -> i32 {
    let logger = logging(get_verbosity_level(), "etrain-checkout");
    
    let default_service = get_default_checkout_source();
    let service_arg = Arg::with_name("service")
                .short("s")
                .long("service")
                .help("Where to checkout from. A lot of cases will be github")
                .default_value(default_service.as_str());

    let matches = App::new("etrain checkout")
        .about("Checkout a project")
        .arg(service_arg)
        .arg(Arg::with_name("repository").help("The (possibly remote) repository to clone from.").required(
            true,
        ))
        .arg(Arg::with_name("directory").help("Clones a repository into a newly created directory."))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .default_value("1")
                .help("Sets the level of verbosity."),
        )
        .get_matches();

    let service = matches.value_of("service").unwrap_or_default();
    let destination = matches.value_of("directory");
    let repository = matches.value_of("repository").unwrap();

    slog_debug!(logger, "Checking out {} from {} into {:?}", repository, service, destination);

    let url = create_url(logger.clone(), service, repository);
    if let Err(e) = url {
        slog_debug!(logger, "Error building URL: {:?}", e);
        return 2;
    }

    let result = do_scm_checkout(logger.clone(), url.unwrap(), destination);

    slog_trace!(logger, "Results from checkout: {:?}", result);

    return match result {
        Ok(value) => value,
        _ => 1,
    };
}
