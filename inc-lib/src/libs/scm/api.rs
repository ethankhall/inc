use core::command::AvaliableCommands;
use libs::scm::provider::git::GitScm;
use libs::scm::services::build_service_map;
use libs::scm::util::compute_destination;
use libs::scm::{CheckoutError, ScmProvier, ScmUrl};

pub fn build_url_from_service(
    service: String,
    user_input: String,
    command: &AvaliableCommands,
    use_ssh: bool,
) -> Result<ScmUrl, CheckoutError> {
    debug!("Origonal input: {}", service);
    let service_map = build_service_map(command);
    let service = service.to_lowercase();
    let service = service.as_str();

    return match service_map.get(service) {
        Some(svc) => svc.generate_url(user_input, use_ssh),
        None => Err(CheckoutError {
            error: format!("Unable to find determine how to execute {}", service),
        }),
    };
}

pub fn build_scm_providers() -> Vec<&'static ScmProvier> {
    return vec![&GitScm {}];
}

pub fn checkout(
    repo_url: &ScmUrl,
    destination: Option<String>,
    providers: Vec<&ScmProvier>,
) -> Result<i32, CheckoutError> {
    let scm_provider = providers.into_iter().find(|x| x.handles_url(repo_url));
    if scm_provider.is_none() {
        return Err(CheckoutError {
            error: format!("Unable to find scm for {}", repo_url),
        });
    }
    let scm_provider = scm_provider.unwrap();

    let suggested_name = scm_provider.sugested_checkout_name(repo_url);
    let destination = compute_destination(destination, suggested_name);

    return scm_provider.do_checkout(repo_url, destination.as_path());
}
