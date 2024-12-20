use std::net::TcpStream;

use crate::http::error::Error;

#[derive(Debug)]
pub struct HTTPRequest {}

impl HTTPRequest {
    pub fn from_stream(stream: &TcpStream) -> Result<Self, Error> {
        Ok(Self {})
    }
}

