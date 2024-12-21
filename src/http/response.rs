use std::io::Write;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use super::header::HTTPHeader;
use super::status::HTTPStatus;
use super::payload::HTTPPayload;
use super::error::Error;

#[derive(Debug)]
pub struct HTTPResponse {
    pub status: HTTPStatus,
    pub payload: Option<HTTPPayload>,
    pub headers: Vec<HTTPHeader>
}

impl HTTPResponse {
    pub fn new(status: HTTPStatus, headers: Vec<HTTPHeader>, payload: Option<HTTPPayload>) -> Self {
        Self { status, payload, headers }
    }

    pub fn to_string(&self) -> String {
        let mut compression = false;
        let mut response = String::new();

        response.push_str(&self.status.to_string());
        response.push_str("\r\n");

        self.headers.iter().for_each(|header| {
            if header.name == "Content-Encoding" && header.value == "gzip" {
                compression = true;
            }
            response.push_str(&header.to_string());
            response.push_str("\r\n");
        });
        response.push_str("\r\n");

        if let Some(payload) = &self.payload {
            if compression {
                response.push_str(&hex::encode(payload.compress().unwrap()));
            } else {
                response.push_str(&payload.to_string());
            }
        }

        response
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut compression = false;
        let mut response = Vec::new();

        response.extend(self.status.to_string().as_bytes());
        response.extend(b"\r\n");

        self.headers.iter().for_each(|header| {
            if header.name == "Content-Encoding" && header.value == "gzip" {
                compression = true;
            }
            response.extend(header.to_string().as_bytes());
            response.extend(b"\r\n");
        });

        if let Some(payload) = &self.payload {
            if compression {
                let compressed_payload = payload.compress().unwrap();
                response.extend(b"Content-Length: ");
                response.extend(compressed_payload.len().to_string().as_bytes());
                response.extend(b"\r\n");
                response.extend(b"\r\n");
                response.extend(compressed_payload);
            } else {
                let uncompressed_payload = payload.as_bytes();
                response.extend(b"Content-Length: ");
                response.extend(uncompressed_payload.len().to_string().as_bytes());
                response.extend(b"\r\n");
                response.extend(b"\r\n");
                response.extend(uncompressed_payload);
            }
        } else {
            response.extend(b"\r\n");
        }

        response
    }

    pub async fn send(&self, stream: &mut TcpStream) -> Result<(), Error> {
        println!("Sending response: {}", self.to_string());
        stream.write_all(&self.encode()).await.unwrap();
        Ok(())
    }
}
