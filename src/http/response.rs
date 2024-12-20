use std::io::Write;
use std::net::TcpStream;

use super::header::HTTPHeader;
use super::status::HTTPStatus;
use super::payload::HTTPPayload;
use super::error::Error;

#[derive(Debug)]
pub struct HTTPResponse {
    status: HTTPStatus,
    payload: Option<HTTPPayload>,
    headers: Vec<HTTPHeader>
}

impl HTTPResponse {
    pub fn new(status: HTTPStatus, headers: Vec<HTTPHeader>, payload: Option<HTTPPayload>) -> Self {
        Self { status, payload, headers }
    }

    pub fn to_string(&self) -> String {
        // vec![&self.status.to_string(), "\r\n", &self.headers.to_string(), "\r\n", &self.payload.to_string()].concat()
        vec![&self.status.to_string(), "\r\n", "\r\n"].concat()
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<(), Error> {
        stream.write_all(self.to_string().as_bytes()).unwrap();
        Ok(())
    }
}
