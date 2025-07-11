pub struct HttpStatus;

impl HttpStatus {
    pub const OK: &str = "200 OK";
    pub const CREATED: &str = "201 Created";
    pub const NOT_FOUND: &str = "404 Not Found";
    pub const INTERNAL_SERVER_ERROR: &str = "500 Internal Server Error";
    pub const NOT_IMPLEMENTED: &str = "501 Not Implemented";
    pub const HTTP_VERSION_NOT_SUPPORTED: &str = "505 HTTP Version Not Supported";
}

pub struct HttpMethod;

impl HttpMethod {
    pub const GET: &str = "GET";
    pub const POST: &str = "POST";
}

pub struct HttpResponse<'a> {
    pub version: &'a str,
    pub status: &'a str,
    pub length: usize,
    pub contents: String,
}

impl<'a> HttpResponse<'a> {
    pub fn new(version: &'a str, status: &'a str) -> Self {
        Self {
            version,
            status,
            length: 0,
            contents: String::new(),
        }
    }
    pub fn response_as_bytes(&self) -> &'a [u8] {
        let formatted_response: &'a str = format!(
            "{} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.version, self.status, self.length, self.contents
        )
        .leak();
        let bytes: &'a [u8] = formatted_response.as_bytes();
        bytes
    }
    pub fn with_body(&mut self, contents: String) -> Self {
        Self {
            version: self.version,
            status: self.status,
            length: contents.len(),
            contents,
        }
    }
}

pub struct HttpRequest<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: &'a str,
}

impl<'a> HttpRequest<'a> {
    pub fn new(request: &'a str) -> Self {
        let lines: Vec<_> = request.split(' ').collect();
        HttpRequest {
            method: lines[0],
            path: lines[1],
            version: lines[2],
        }
    }
}
