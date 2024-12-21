use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::http::error::Error;

use super::{header::HTTPHeader, method::HTTPMethod, payload::HTTPPayload};

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub version: String,
    pub headers: Vec<HTTPHeader>,
    pub body: Option<HTTPPayload>,
}

impl HTTPRequest {
    pub async fn from_stream(stream: &mut TcpStream) -> Result<Self, Error> {
        let mut buf = [0; 1024];
        let len = stream.peek(&mut buf).await.expect("peek failed");
        
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();

        let request = String::from_utf8(buffer[..len].to_vec()).unwrap();
        let mut request_lines = request.split("\r\n");

        // parse request line
        let request_line = request_lines.next().unwrap();
        let method = HTTPMethod::from_str(request_line.split(" ").nth(0).unwrap()).unwrap();
        let path = request_line.split(" ").nth(1).unwrap().to_string();
        let version = request_line.split(" ").nth(2).unwrap().to_string();

        // parse headers
        let mut headers = Vec::new();
        while let Some(line) = request_lines.next() {
            if line.is_empty() {
                break;
            }
            let header = HTTPHeader::from(line);
            headers.push(header);
        }

        // parse body
        let body = {
            if let Some(line) = request_lines.next() {
                if !line.is_empty() {
                    Some(HTTPPayload::new(line.to_string()))
                } else {
                    None
                }
            } else {
                None
            }
        };

        Ok(Self {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    pub fn get_path_parts(&self) -> Vec<&str> {
        self.path.split("/").collect::<Vec<&str>>()
    }

    pub fn get_header(&self, name: &str) -> Option<&HTTPHeader> {
        self.headers
            .iter()
            .find(|header| header.name == name)
    }
}
