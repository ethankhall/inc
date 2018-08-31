use inc_lib::core::command::AvaliableCommands;
use inc_lib::libs::scm::api::{build_scm_providers, build_url_from_service, checkout};
use inc_lib::core::BASE_APPLICATION_NAME;
use std::collections::HashSet;
use inc_lib::libs::scm::{DEFAULT_CHECKOUT_SOURCE, PRE_DEFINED_CHECKOUT_SOURCES};
use inc_lib::exec::executor::CliResult;
use std::vec::Vec;
use clap::{App, Arg, ArgMatches, SubCommand};
use inc_lib::core::config::ConfigContainer;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    return SubCommand::with_name("checkout")
        .about("Checkout from SCM")
        .arg(
            Arg::with_name("service")
                .long("service")
                .short("s")
                .help("Where to checkout from. A lot of cases will be github.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list-services")
                .long("list-services")
                .help("List all of the avaliable services."),
        )
        .arg(
            Arg::with_name("https-only")
                .long("https-only")
                .help("Only do checkouts using http instead of ssh."),
        )
        .arg(
            Arg::with_name("repository")
                .help("The (possibly remote) repository to clone from.")
                .takes_value(true)
                .required(true)
                .required_unless("list-services"),
        )
        .arg(
            Arg::with_name("directory")
                .help("Clones a repository into a newly created directory.")
                .takes_value(true),
        );
}

fn get_default_checkout_service(config: ConfigContainer) -> String {
    return config
        .get_home_configs()
        .checkout
        .default_provider
        .unwrap_or_else(|| String::from(DEFAULT_CHECKOUT_SOURCE));
}

pub fn execute(
    args: &ArgMatches,
    commands: AvaliableCommands,
    config: ConfigContainer,
) -> CliResult {
    let mut service_options = possible_checkout_sources(&commands);
    service_options.sort();

    let default_sources = get_default_checkout_service(config);
    trace!("Avaliable checkout sources: {:?}", service_options);

    if args.is_present("list-services") {
        let mut service_list: Vec<String> = Vec::new();
        for service in service_options.into_iter() {
            let mut body = String::from(format!(" - {}", service));
            if default_sources == service {
                body.push_str("\t[default]");
            }
            service_list.push(body);
        }
        info!("Services:\n{}", service_list.join("\n"));
        return Ok(0);
    }

    let service = args.value_of("service").unwrap_or_else(|| &default_sources);
    let destination = args.value_of("directory").map(|x| s!(x));
    let repository = args.value_of("repository").unwrap();

    debug!(
        "Checking out {}, using {} to get url, into {:?}",
        repository, service, destination
    );

    let scm_providers = build_scm_providers();

    let url = if scm_providers
        .clone()
        .into_iter()
        .any(|x| x.handles_url(&s!(repository)))
    {
        Ok(s!(repository))
    } else {
        build_url_from_service(
            s!(service),
            s!(repository),
            &commands,
            !args.is_present("https-only"),
        )
    };

    if let Err(e) = url {
        debug!("Error building URL: {:?}", e);
        error!("Unable to determine URL. Error: {:?}", e.error);
        return Ok(2);
    }

    let url = url.unwrap();

    debug!("Url to checkout is: {:?}", url);

    let result = checkout(&url, destination, scm_providers);

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

fn possible_checkout_sources(commands: &AvaliableCommands) -> Vec<String> {
    let mut avaliable_sources: HashSet<String> = HashSet::new();

    for existing in PRE_DEFINED_CHECKOUT_SOURCES {
        avaliable_sources.insert(String::from(*existing));
    }

    for command in
        commands.find_commands_with_parent(format!("{}-checkout-service", BASE_APPLICATION_NAME))
    {
        avaliable_sources.insert(command.name());
    }

    let mut avaliable_sources: Vec<String> = avaliable_sources.into_iter().collect();
    avaliable_sources.sort();

    return avaliable_sources;
}
