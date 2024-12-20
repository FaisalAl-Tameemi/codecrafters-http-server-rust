
use std::collections::HashMap;
use std::net::TcpStream;

use super::error::Error;
use super::response::HTTPResponse;
use super::status::{HTTPStatus, HTTPStatusCode};

pub type HTTPHandler = Box<dyn Fn() -> Result<HTTPResponse, Error>>;

pub struct HTTPRouter {
    routes: HashMap<&'static str, HTTPHandler>,
}

impl HTTPRouter {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    pub fn add_route(&mut self, path: &'static str, handler: HTTPHandler) {
        self.routes.insert(path, handler);
    }

    pub fn handle_request(&self, path: &str, stream: &mut TcpStream) -> Result<HTTPResponse, Error> {
        if let Some(handler) = self.routes.get(path) {
            handler()
        } else {
            Ok(HTTPResponse::new(HTTPStatus::new(HTTPStatusCode::NOT_FOUND, "1.1".to_string()), vec![], None))
        }
    }
}
