use scm;
use slog::Logger;
use etrain_core::command::MainCommand;
use etrain_core::logging::{logging, get_verbosity_level};
use etrain_core::config::{ConfigParser, ConfigContainer, ConfigSource, ConfigValue};
use scm::core::{do_scm_checkout, create_url};
use std::process;
use etrain_core::cli::CliResolver;
use etrain_core::BASE_APPLICATION_NAME;
use std::collections::HashSet;
use docopt::Docopt;
use {PRE_DEFINED_CHECKOUT_SOURCES, DEFAULT_CHECKOUT_SOURCE};

#[derive(Deserialize, Debug)]
struct Args {
    arg_repository: String,
    arg_directory: Option<String>,
    flag_version: bool,
    flag_help: bool,
    flag_verbose: Option<String>,
    flag_service: Option<String>,
}

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

fn possible_checkout_sources(commands: Vec<String>) -> Vec<String> {
    let mut avaliable_sources: HashSet<String> = HashSet::new();

    for existing in PRE_DEFINED_CHECKOUT_SOURCES {
        avaliable_sources.insert(String::from(*existing));
    }

    for external_source in commands.into_iter() {
        avaliable_sources.insert(external_source);
    }

    return avaliable_sources.into_iter().collect();
}

pub fn build_checkout_command() -> impl MainCommand {
    return CheckoutCommand{};
}

struct CheckoutCommand {}

impl MainCommand for CheckoutCommand {
    fn execute(&self, args: Vec<String>, logger: &Logger, config_container: &ConfigContainer, sub_commands: Vec<String>) -> i32 {
        let service_options = possible_checkout_sources(sub_commands);
        let default_sources = config_container.get_from_source_default(String::from("checkout.default"), ConfigSource::Home, String::from(DEFAULT_CHECKOUT_SOURCE));
        let doc_opts: Args = Docopt::new(build_usage(default_sources, service_options))
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

    fn get_command_name(&self) -> String {
        return String::from("checkout");
    }

    fn get_command_prefix(&self) -> String {
        return format!("{}-{}", BASE_APPLICATION_NAME, self.get_command_name());
    }

    fn get_description(&self) -> String {
        return String::from("Checkout a repo from source control");
    }
}