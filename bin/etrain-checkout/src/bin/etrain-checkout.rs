#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
extern crate etrain_core;
extern crate checkout;
extern crate docopt;

use etrain_core::logging::{logging, get_verbosity_level};
use etrain_core::config::{ConfigParser, ConfigContainer, ConfigSource, ConfigValue};
use checkout::scm::{do_scm_checkout, create_url};
use std::process;
use etrain_core::cli::CliResolver;
use etrain_core::BASE_APPLICATION_NAME;
use std::collections::HashSet;
use docopt::Docopt;

#[derive(Deserialize)]
struct Args {
    arg_repository: String,
    arg_directory: Option<String>,
    flag_version: bool,
    flag_help: bool,
    flag_verbose: Option<String>,
    flag_service: Option<String>,
}

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

const DEFAULT_CHECKOUT_SOURCE: &'static str = "github";
const PRE_DEFINED_CHECKOUT_SOURCES: &'static [&'static str] = &["github"];

fn build_usage(default_service: String, service_options: Vec<String>) -> String {
    let service_options = service_options.join(", ");

    return format!("Usage:
    etrain-checkout [--service=<service>] <repository> [<directory>]
    etrain-checkout [--verbose=<level>] <repository> [<directory>]
    etrain-checkout (-h | --help)
    etrain-checkout (-V | --version)

Flags:
    -h, --help       Prints help information
    -V, --version    Prints version information

Options:
    -s, --service <service>    Where to checkout from. A lot of cases will be github. [ default: {default} ] [ options: {services} ]
    -v, --verbose <verbose>    Sets the level of verbosity. [ default: 1 ]

Args:
    <repository>    The (possibly remote) repository to clone from.
    <directory>     Clones a repository into a newly created directory.
", default = default_service, services = service_options);
}

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

fn possible_checkout_sources(commands: HashSet<String>) -> Vec<String> {
    let mut avaliable_sources: HashSet<String> = HashSet::new();

    for existing in PRE_DEFINED_CHECKOUT_SOURCES {
        avaliable_sources.insert(String::from(*existing));
    }

    for external_source in commands.into_iter() {
        avaliable_sources.insert(external_source);
    }

    return avaliable_sources.into_iter().collect();
}

fn do_main() -> i32 {
    let prefix_string = format!("{}-checkout", BASE_APPLICATION_NAME);
    let logger = logging(get_verbosity_level(), prefix_string.clone());
    
    let cli_resolver = CliResolver { logger: logger.clone(), prefix: prefix_string};

    let service_options = possible_checkout_sources(cli_resolver.find_commands());
    let doc_opts: Args = Docopt::new(build_usage(get_default_checkout_source(), service_options))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let service = doc_opts.flag_service.unwrap_or_else(|| { String::from(DEFAULT_CHECKOUT_SOURCE) });
    let destination = doc_opts.arg_directory;
    let repository = doc_opts.arg_repository;

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
