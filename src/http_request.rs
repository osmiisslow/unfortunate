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
