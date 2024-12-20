use std::io::Write;
use std::net::TcpListener;

mod http;
use http::request::HTTPRequest;
use http::response::HTTPResponse;
use http::router::HTTPRouter;
use http::status::{HTTPStatus, HTTPStatusCode};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let mut router = HTTPRouter::new();
                
    router.add_route("/", Box::new(|| {
        let response = HTTPResponse::new(
            HTTPStatus::new(HTTPStatusCode::OK, "1.1".to_string()),
            vec![],
            None
        );
        Ok(response)
    }));
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Incoming connection from {:?}", stream.peer_addr().unwrap());

                let request = HTTPRequest::from_stream(&mut stream).unwrap();

                match router.handle_request(&request.path, &mut stream) {
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
