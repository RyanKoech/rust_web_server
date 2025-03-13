use crate::server::Handler;
use crate::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct WebsiteHandler {
  public_path: String,
}

impl WebsiteHandler {
  pub fn new(public_path: String) -> Self {
      Self { public_path }
  }

  fn read_file(&self, file_path: &str) -> Option<String> {
    let path = format!("{}/{}", self.public_path, file_path);
    fs::read_to_string(path).ok()
  }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> crate::http::Response {
      match request.method() {
        Method::GET => match request.path() {
            "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
            _ => Response::new(StatusCode::NotFound, None),
        },
        _ => Response::new(StatusCode::NotFound, None),
      }
    }
}