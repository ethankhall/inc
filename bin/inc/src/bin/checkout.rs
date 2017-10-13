use inc_core::core::command::{CommandContainer};
use inc_core::core::config::ConfigContainer;
use inc_core::libs::scm::api::{build_url_from_service, checkout};
use inc_core::core::BASE_APPLICATION_NAME;
use std::collections::HashSet;
use inc_core::libs::scm::{PRE_DEFINED_CHECKOUT_SOURCES, DEFAULT_CHECKOUT_SOURCE};
use inc_core::libs::process::SystemBinary;
use inc_core::exec::executor::{CliResult};

#[derive(Deserialize, Debug)]
pub(crate) struct Options {
    arg_repository: String,
    arg_directory: Option<String>,
    flag_version: bool,
    flag_help: bool,
    flag_verbose: Option<String>,
    flag_service: Option<String>,
}

pub const USAGE: &'static str = "Usage:
  inc-checkout [options] <repository> [<directory>]
  inc-checkout (-h | --help)

Options:
    -s <service>, --service=<service>       Where to checkout from. A lot of cases will be github.
    -v, --verbose ...                       Increasing verbosity.
    -w, --warn                              Only display warning messages.
    -q, --quiet                             No output printed to stdout.
    -h, --help                              Prints this message.
    -l, --list                              Lists all options for service.

Args:
  <repository>    The (possibly remote) repository to clone from.
  <directory>     Clones a repository into a newly created directory.";

pub(crate) fn execute(options: Options) -> CliResult {
    trace!("Arguments to checkout: {:?}", options);

    let command_container = CommandContainer::new();
    let config_container = ConfigContainer::new();

    let sub_commands = match command_container.find_sub_commands(format!("{}-checkout", BASE_APPLICATION_NAME)) {
        Some(value) => value.sub_commands,
        None => Vec::new(),
    };

    let service_options = possible_checkout_sources(&sub_commands);
    trace!("Avaliable checout sources: {:?}", service_options);

    let default_sources = config_container
        .get_checkout_configs()
        .default
        .unwrap_or_else(|| String::from(DEFAULT_CHECKOUT_SOURCE));

    let service = options.flag_service.unwrap_or_else(|| default_sources);
    let destination = options.arg_directory;
    let repository = options.arg_repository;

    debug!(
        "Checking out {}, using {} to get url, into {:?}",
        repository,
        service,
        destination
    );

    let url = build_url_from_service(
        service,
        repository,
        &sub_commands,
    );

    if let Err(e) = url {
        debug!("Error building URL: {:?}", e);
        error!("Unable to determine URL. Error: {:?}", e.error);
        return Ok(2);
    }

    let url = url.unwrap();

    debug!("Url to checkout is: {:?}", url);

    let result = checkout(&url, destination);

    trace!("Results from checkout: {:?}", result);
    if result.is_err() {
        error!("Unable to checkout from {:?}", url);
    }

    let code = match result {
        Ok(value) => value,
        _ => 1,
    };

    return Ok(code);
}

fn possible_checkout_sources(commands: &Vec<SystemBinary>) -> Vec<String> {
    let mut avaliable_sources: HashSet<String> = HashSet::new();

    for existing in PRE_DEFINED_CHECKOUT_SOURCES {
        avaliable_sources.insert(String::from(*existing));
    }

    let service_prefix = format!("{}-checkout-service-", BASE_APPLICATION_NAME);

    for external_source in commands.into_iter() {
        if external_source.name.starts_with(service_prefix.as_str()) {
            avaliable_sources.insert(String::from(
                &external_source.name[(service_prefix.len())..],
            ));
        }
    }

    return avaliable_sources.into_iter().collect();
}