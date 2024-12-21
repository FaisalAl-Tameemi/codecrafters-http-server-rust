use std::sync::Arc;
use tokio::net::TcpListener;

mod http;
use http::header::HTTPHeader;
use http::payload::HTTPPayload;
use http::request::HTTPRequest;
use http::response::HTTPResponse;
use http::router::HTTPRouter;
use http::status::{HTTPStatus, HTTPStatusCode};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
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

    let router = Arc::new(router);
    
    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                let router = Arc::clone(&router);
                
                tokio::spawn(async move {
                    let request = HTTPRequest::from_stream(&mut stream).await.unwrap();
                    match router.handle_request(&request) {
                        Ok(response) => {
                            response.send(&mut stream).await.unwrap();
                        },
                        Err(e) => println!("error: {}", e),
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
