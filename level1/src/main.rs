use http::{Response, Error};
use http::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH};
use std::fmt;

struct DisplayResponse(Response<String>);

impl fmt::Display for DisplayResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let headers = self.0.headers();
        let status = self.0.status();
        let body = self.0.body();
        write!(f, "HTTP/1.1 {status}\r\n")?;
        for (key, value) in headers {
            write!(f, "{}: {}\r\n", key.as_str(), value.to_str().unwrap_or(""))?;
        }
        write!(f, "\r\n{}", body)
    }
}

fn main() -> Result<(), Error> {
    let response = build_response(200, "Pong!".into())?;
    println!("{response}");
    Ok(())
}

fn build_response(status: u16, body: String) -> Result<DisplayResponse, Error> {
    Ok(DisplayResponse(Response::builder()
        .status(status)
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .header(CONTENT_LENGTH, HeaderValue::from_str(&body.as_bytes().len().to_string())?)
        .body(body)?))
}
