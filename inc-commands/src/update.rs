use http::{Request, Method};
use inc_lib::libs::http::{set_default_headers};

pub fn do_update() {
    let mut request = Request::new(Method::GET, "https://api.github.com/repos/ethankhall/inc/releases/latest");
    set_default_headers(request.headers_mut());
}