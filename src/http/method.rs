use super::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE
}

impl HTTPMethod {
    pub fn from_str(method: &str) -> Result<Self, Error> {
        match method {
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            "PUT" => Ok(HTTPMethod::PUT),
            "DELETE" => Ok(HTTPMethod::DELETE),
            _ => Err(Error::InvalidMethod)
        }
    }
}