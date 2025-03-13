use crate::server::Handler;
use crate::http::{Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> crate::http::Response {
        Response::new(StatusCode::Ok, Some("<h1> Hello From Rust Server!! </h1>".to_string()))
    }
}