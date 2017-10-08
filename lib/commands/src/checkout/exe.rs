use inc_core::core::command::{MainCommand, CommandContainer};
use inc_core::core::config::{ConfigContainer, ConfigSource};
use inc_core::libs::scm::api::{build_url_from_service, checkout};
use inc_core::core::BASE_APPLICATION_NAME;
use std::collections::HashSet;
use docopt::Docopt;
use inc_core::libs::scm::{PRE_DEFINED_CHECKOUT_SOURCES, DEFAULT_CHECKOUT_SOURCE};
use inc_core::libs::process::SystemBinary;
use inc_core::exec::Execution;

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

    return format!(
        "Usage:
    inc-checkout [<options>] <repository> [<directory>]
    inc-checkout (-h | --help)
    inc-checkout (-V | --version)

Flags:
    -h, --help       Prints help information
    -V, --version    Prints version information

Options:
    -s, --service <service>    Where to checkout from. \
A lot of cases will be github. [ default: {default} ] [ options: {services} ]
    -v <level>, --verbose=<level>    Sets the level of verbosity. [ default: 1 ]

Args:
    <repository>    The (possibly remote) repository to clone from.
    <directory>     Clones a repository into a newly created directory.
",
        default = default_service,
        services = service_options
    );
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

pub struct CheckoutCommand {
    pub config_container: ConfigContainer,
    pub command_container: CommandContainer
}

impl CheckoutCommand {
    fn my_execute(&self, args: &Vec<String>) -> i32 {

        let sub_commands = match self.command_container.find_sub_commands(self.get_command_prefix()) {
            Some(value) => value.sub_commands,
            None => Vec::new(),
        };

        let service_options = possible_checkout_sources(&sub_commands);
        trace!("Avaliable checout sources: {:?}", service_options);

        let default_sources = self.config_container.get_from_source_default(
            String::from("checkout.default"),
            ConfigSource::Home,
            String::from(DEFAULT_CHECKOUT_SOURCE),
        );
        let doc_opts: Args = Docopt::new(build_usage(default_sources, service_options))
            .and_then(|d| d.argv(args.into_iter()).parse())
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        let service = doc_opts.flag_service.unwrap_or_else(|| {
            String::from(DEFAULT_CHECKOUT_SOURCE)
        });
        let destination = doc_opts.arg_directory;
        let repository = doc_opts.arg_repository;

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
            return 2;
        }

        let url = url.unwrap();

        debug!("Url to checkout is: {:?}", url);

        let result = checkout(url, destination);

        trace!("Results from checkout: {:?}", result);

        return match result {
            Ok(value) => value,
            _ => 1,
        };
    }
}

impl Execution<i32> for CheckoutCommand {
    fn execute(&self, args: &Vec<String>) -> Result<i32, String> {
        return Ok(self.my_execute(args));
    }
}

impl MainCommand for CheckoutCommand {
    fn execute(&self, args: &Vec<String>) -> i32 {
        return self.my_execute(args);
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
