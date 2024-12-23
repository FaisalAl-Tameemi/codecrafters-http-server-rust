use std::sync::Arc;
use http::error::Error;
use http::method::HTTPMethod;
use tokio::net::TcpListener;
use std::fs;
use clap::Parser;

mod http;
use http::header::HTTPHeader;
use http::payload::HTTPPayload;
use http::request::HTTPRequest;
use http::response::HTTPResponse;
use http::router::HTTPRouter;
use http::status::{HTTPStatus, HTTPStatusCode};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    directory: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    let mut router = HTTPRouter::new(args.directory);

    router.add_route(HTTPMethod::GET, "/", Box::new(|_, _, _| {
        let response = HTTPResponse::new(
            HTTPStatus::new(HTTPStatusCode::OK, "1.1".to_string()),
            vec![],
            None
        );
        Ok(response)
    }));

    router.add_route(HTTPMethod::GET, "/echo/{message}", Box::new(|params, _, _| {
        let response = HTTPResponse::new(
            HTTPStatus::new(HTTPStatusCode::OK, "1.1".to_string()),
            vec![
                HTTPHeader::new("Content-Type".to_string(), "text/plain".to_string()),
            ],
            Some(HTTPPayload::new(params["message"].to_string()))
        );
        Ok(response)
    }));

    router.add_route(HTTPMethod::GET, "/user-agent", Box::new(|_, request, _| {
        let user_agent = request.get_header("User-Agent").expect("User-Agent header is required");
        let response = HTTPResponse::new(
            HTTPStatus::new(HTTPStatusCode::OK, "1.1".to_string()),
            vec![
                HTTPHeader::new("Content-Type".to_string(), "text/plain".to_string()),
            ],
            Some(HTTPPayload::new(user_agent.value.to_string()))
        );
        Ok(response)
    }));

    router.add_route(HTTPMethod::GET, "/files/{file}", Box::new(|params, _, options| {
        let directory = match options.first().unwrap() {
            Some(directory) => directory,
            None => return Err(Error::DirectoryNotSet),
        };
        let file_path = String::new() + &directory + "/" + &params["file"];
        let file_content = fs::read(file_path);
        
        match file_content {
            Ok(file_content) => {
                let response = HTTPResponse::new(
                    HTTPStatus::new(HTTPStatusCode::OK, "1.1".to_string()),
                    vec![
                        HTTPHeader::new("Content-Type".to_string(), "application/octet-stream".to_string()),
                    ],
                    Some(HTTPPayload::new(String::from_utf8(file_content).unwrap()))
                );
                Ok(response)
            }
            Err(_) => {
                let response = HTTPResponse::new(
                    HTTPStatus::new(HTTPStatusCode::NotFound, "1.1".to_string()),
                    vec![],
                    None
                );
                Ok(response)
            }
        }
    }));

    router.add_route(HTTPMethod::POST, "/files/{file}", Box::new(|params, request, options| {
        let directory = match options.first().unwrap() {
            Some(directory) => directory,
            None => return Err(Error::DirectoryNotSet),
        };
        let file_path = String::new() + &directory + "/" + &params["file"];
        let file_content = request.body.as_ref().unwrap().to_string();
        fs::write(file_path, file_content).unwrap();
        let response = HTTPResponse::new(
            HTTPStatus::new(HTTPStatusCode::Created, "1.1".to_string()),
            vec![],
            None
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
                        Err(e) => println!("unhandled server error: {}", e),
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
