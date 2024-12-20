use std::net::TcpListener;

mod http;
use http::header::HTTPHeader;
use http::payload::HTTPPayload;
use http::request::HTTPRequest;
use http::response::HTTPResponse;
use http::router::HTTPRouter;
use http::status::{HTTPStatus, HTTPStatusCode};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let mut router = HTTPRouter::new();
                
    router.add_route("/", Box::new(|_, _| {
        let response = HTTPResponse::new(
            HTTPStatus::new(HTTPStatusCode::OK, "1.1".to_string()),
            vec![],
            None
        );
        Ok(response)
    }));

    router.add_route("/echo/{message}", Box::new(|params, _| {
        let response = HTTPResponse::new(
            HTTPStatus::new(HTTPStatusCode::OK, "1.1".to_string()),
            vec![
                HTTPHeader::new("Content-Type".to_string(), "text/plain".to_string()),
                HTTPHeader::new("Content-Length".to_string(), params["message"].len().to_string()),
            ],
            Some(HTTPPayload::new(params["message"].to_string()))
        );
        Ok(response)
    }));

    router.add_route("/user-agent", Box::new(|_, request| {
        let user_agent = request.get_header("User-Agent").expect("User-Agent header is required");
        let response = HTTPResponse::new(
            HTTPStatus::new(HTTPStatusCode::OK, "1.1".to_string()),
            vec![
                HTTPHeader::new("Content-Type".to_string(), "text/plain".to_string()),
                HTTPHeader::new("Content-Length".to_string(), user_agent.value.len().to_string()),
            ],
            Some(HTTPPayload::new(user_agent.value.to_string()))
        );
        Ok(response)
    }));
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let request = HTTPRequest::from_stream(&mut stream).unwrap();

                println!("Received request: {:?}", request);

                match router.handle_request(&request) {
                    Ok(response) => {
                        response.send(&mut stream).unwrap();
                    },
                    Err(e) => println!("error: {}", e),
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
