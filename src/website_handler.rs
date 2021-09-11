use super::server::Handler;
use crate::http::{Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    Response::new(StatusCode::Ok, Some("<h3>Toets 1 2 3</h3>".to_string()))
  }
}
