
#[derive(Debug)]
pub enum HTTPStatusCode {
    OK = 200,
    Created = 201,
    NotFound = 404,
    InternalServerError = 500
}

impl HTTPStatusCode {
    pub fn as_str(&self) -> &str {
        match self {
            HTTPStatusCode::OK => "200 OK",
            HTTPStatusCode::Created => "201 Created",
            HTTPStatusCode::NotFound => "404 Not Found",
            HTTPStatusCode::InternalServerError => "500 Internal Server Error"
        }
    }
}

#[derive(Debug)]
pub struct HTTPStatus {
    status_code: HTTPStatusCode,
    version: String
}

impl HTTPStatus {
    pub fn new(status_code: HTTPStatusCode, version: String) -> Self {
        Self { status_code, version }
    }

    pub fn to_string(&self) -> String {
        ["HTTP/", self.version.as_str(), " ", self.status_code.as_str()].concat()
    }
}

