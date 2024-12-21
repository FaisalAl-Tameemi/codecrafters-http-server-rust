use std::collections::HashMap;
use regex::Regex;

use super::error::Error;
use super::request::HTTPRequest;
use super::response::HTTPResponse;
use super::status::{HTTPStatus, HTTPStatusCode};

pub type HTTPHandler = 
    Box<dyn Fn(HashMap<&'static str, String>, &HTTPRequest) -> 
        Result<HTTPResponse, Error> + Send + Sync>;

pub struct Route {
    path: &'static str,
    handler: HTTPHandler,
    params: HashMap<&'static str, usize>,
}

impl Route {
    pub fn parse_params(&self, request_path: &str) -> HashMap<&'static str, String> {
        let request_path_parts = request_path.split("/").collect::<Vec<&str>>();
        let mut params = HashMap::<&'static str, String>::new();
        self.params.iter().for_each(|(key, value)| {
            params.insert(key, request_path_parts.get(*value).unwrap().to_string());
        });
        params
    }
}

pub struct HTTPRouter {
    // a map of pattern and the route
    routes: Vec<Route>,
}

impl HTTPRouter {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, path: &'static str, handler: HTTPHandler) {
        // use regex to extract the path parameters, currently only supports one parameter
        let re = Regex::new(r"\{(\w+)\}").unwrap();
        let path_parts = path.split("/").collect::<Vec<&str>>();
        // hash map of parameter name and index in the path
        let mut params = HashMap::<&str, usize>::new();
        
        path_parts.iter().enumerate().for_each(|(index, part)| {
            let captures = re.captures(part);
            if captures.is_some() {
                let captures = captures.unwrap();
                let captures = captures.iter().map(|c| c.unwrap().as_str()).collect::<Vec<&str>>();
                params.insert(captures.get(1).unwrap(), index);
            }
        });

        self.routes.push(Route {
            path,
            handler,
            params
        });
    }

    pub fn handle_request(&self, request: &HTTPRequest) -> Result<HTTPResponse, Error> {
        let request_path_parts = request.get_path_parts();
        let matched_route = self.routes.iter().find(|route| {
            // If paths are exactly equal, it's a direct match
            if route.path == request.path {
                return true;
            }

            // For parameterized routes, we need more careful matching
            let route_path_parts: Vec<&str> = route.path.split("/").collect();
            
            // Must have same number of path segments
            if route_path_parts.len() != request_path_parts.len() {
                return false;
            }

            // Check each path segment
            route_path_parts.iter().zip(request_path_parts.iter()).all(|(route_part, request_part)| {
                // If route part is a parameter (wrapped in {}), it matches any value
                if route_part.starts_with("{") && route_part.ends_with("}") {
                    true
                } else {
                    // For non-parameter segments, must match exactly
                    route_part == request_part
                }
            })
        });

        if let Some(route) = matched_route {
            let params = route.parse_params(&request.path);
            (route.handler)(params, request)
        } else {
            Ok(HTTPResponse::new(
                HTTPStatus::new(
                    HTTPStatusCode::NOT_FOUND,
                    "1.1".to_string()
                ),
                vec![],
                None
            ))
        }
    }
}
