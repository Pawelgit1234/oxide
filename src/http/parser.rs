use httparse::{Request, Status};

pub fn parse_request(buf: &[u8]) -> Result<(String, String), &'static str> {
    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut req = Request::new(&mut headers);

    match req.parse(buf) {
        Ok(Status::Complete(_)) => {
            let method = req.method.ok_or("Missing method")?.to_string();
            let path = req.path.ok_or("Missing path")?.to_string();
            Ok((method, path))
        }
        Ok(Status::Partial) => Err("Incomplete request"),
        Err(_) => Err("Failed to parse request"),
    }
}