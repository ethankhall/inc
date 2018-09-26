use std::boxed::Box;
use std::error::Error;
use std::ops::Deref;

use futures::{future, Future, Stream};
use hyper::Error as HyperError;
use hyper::client::HttpConnector;
use hyper::header::{qitem, Accept, Authorization, Headers, UserAgent};
use hyper::mime::Mime;
use hyper::{Client, Request, StatusCode};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

use indicatif::{ProgressBar, ProgressStyle};

struct DefaultHttpRequester {
    show_spinner: bool
}

impl DefaultHttpRequester {
    pub fn new(hide_spinner: bool) -> Self {
        return DefaultHttpRequester { show_spinner: !hide_spinner};
    }

    fn make_external_parts<'a>(&self) -> (Core, Client<HttpsConnector<HttpConnector>>) {
        let core = Core::new().expect("Unable to create HTTP Core");
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).unwrap())
            .build(&core.handle());

        return (core, client);
    }
}

pub trait HttpRequester {
    fn make_request(&self, request: Request) -> Result<(StatusCode, String), ErrorCodes>;
}

pub fn set_default_headers(headers: &mut Headers) {
    let user_agent = UserAgent::new(format!("inc/{}", env!("CARGO_PKG_VERSION")));
    headers.set(user_agent);
}

impl HttpRequester for DefaultHttpRequester {
    fn make_request(&self, request: Request) -> Result<(StatusCode, String), ErrorCodes> {
        trace!("Request to be sent: {:?}", &request);

        let spinner = ProgressBar::new_spinner();
        if self.show_spinner {
            spinner.set_style(ProgressStyle::default_spinner()
                .tick_chars("/|\\- ")
                .template("{spinner:.dim.bold} Processing request to {wide_msg}"));
            spinner.enable_steady_tick(100);
            spinner.tick();
            spinner.set_message(&format!("{}", request.uri()));
        }

        let (mut core, client) = self.make_external_parts();
        let work = client.request(request).and_then(|res| {
            let status = Box::new(res.status());

            res.body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, HyperError>(v)
                })
                .and_then(|chunks| {
                    let bdy = String::from_utf8(chunks).unwrap();
                    future::ok::<_, HyperError>((status, s!(bdy)))
                })
        });

        let (status, body) = match core.run(work) {
            Ok((status, body)) => (status, String::from(body)),
            Err(err) => {
                trace!("Request Error: {:?}", err);
                error!("Unable to make request becasue `{}`", err.description());
                spinner.finish_and_clear();
                return Err(ErrorCodes::NetworkCallFailed);
            }
        };

        if show_spinner {
            spinner.finish_and_clear();
        }

        trace!("Body from API: {}", body);

        return Ok((*status.deref(), body));
    }
}