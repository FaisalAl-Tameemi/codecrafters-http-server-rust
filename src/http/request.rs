use std::{io::Read, net::TcpStream};

use crate::http::error::Error;

use super::{header::HTTPHeader, payload::HTTPPayload};

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<HTTPHeader>,
    pub body: Option<HTTPPayload>
}

impl HTTPRequest {
    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, Error> {
        let mut buf = [0; 1024];
        let len = stream.peek(&mut buf).expect("peek failed");
        
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8(buffer[..len].to_vec()).unwrap();
        let mut request_lines = request.split("\r\n");

        let request_line = request_lines.next().unwrap();
        let method = request_line.split(" ").nth(0).unwrap().to_string();
        let path = request_line.split(" ").nth(1).unwrap().to_string();
        let version = request_line.split(" ").nth(2).unwrap().to_string();

        Ok(Self {
            method,
            path,
            version,
            headers: vec![],
            body: None
        })
    }

    pub fn get_path_parts(&self) -> Vec<&str> {
        self.path.split("/").collect::<Vec<&str>>()
    }
}

