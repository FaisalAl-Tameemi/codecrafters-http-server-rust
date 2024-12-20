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
        let mut response = String::new();

        response.push_str(&self.status.to_string());
        response.push_str("\r\n");

        self.headers.iter().for_each(|header| {
            response.push_str(&header.to_string());
            response.push_str("\r\n");
        });
        response.push_str("\r\n");

        if let Some(payload) = &self.payload {
            response.push_str(&payload.to_string());
        }

        response
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<(), Error> {
        stream.write_all(self.to_string().as_bytes()).unwrap();
        Ok(())
    }
}
