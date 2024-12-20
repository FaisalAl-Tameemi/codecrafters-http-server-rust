
#[derive(Debug)]
pub enum HTTPStatusCode {
    OK = 200,
    NOT_FOUND = 404,
    INTERNAL_SERVER_ERROR = 500
}

impl HTTPStatusCode {
    pub fn as_str(&self) -> &str {
        match self {
            HTTPStatusCode::OK => "200 OK",
            HTTPStatusCode::NOT_FOUND => "404 Not Found",
            HTTPStatusCode::INTERNAL_SERVER_ERROR => "500 Internal Server Error"
        }
    }
}

#[derive(Debug)]
pub struct HTTPStatus {
    statusCode: HTTPStatusCode,
    version: String
}

impl HTTPStatus {
    pub fn new(statusCode: HTTPStatusCode, version: String) -> Self {
        Self { statusCode, version }
    }

    pub fn to_string(&self) -> String {
        ["HTTP/", self.version.as_str(), " ", self.statusCode.as_str()].concat()
    }
}

