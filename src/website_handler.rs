use crate::server::Handler;
use crate::http::{Request, Response, StatusCode, Method};


pub struct WebsiteHandler {
  public_path: String,
}

impl WebsiteHandler {
  pub fn new(public_path: String) -> Self {
      Self { public_path }
  }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> crate::http::Response {
      match request.method() {
        Method::GET => match request.path() {
            "/" => Response::new(StatusCode::Ok, Some("<h1> Hello From Rust Server!! </h1>".to_string())),
            _ => Response::new(StatusCode::NotFound, None),
        },
        _ => Response::new(StatusCode::NotFound, None),
      }
    }
}