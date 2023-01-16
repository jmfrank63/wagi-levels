use http::{Response, Error, StatusCode};
use http::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH};
use std::fmt;

struct WagiResponse<B>(http::Response<B>);

impl<B> fmt::Display for WagiResponse<B>
    where B: fmt::Display
{
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
    // let response = build_response(200, "Pong!".into())?;
    let body = "Pong!";
    let status = StatusCode::OK;
    let response = Response::builder()
        .version(http::Version::HTTP_11)
        .status(status)
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .header(CONTENT_LENGTH, HeaderValue::from_str(&body.as_bytes().len().to_string())?)
        .body(body)?;
    let (parts, body) = response.into_parts();
    println!("{:?} {}", parts.version, parts.status);
    for (key, value) in parts.headers {
        println!("{}: {}\r\n", key.unwrap().as_str(), value.to_str().unwrap());
    }
    println!("\r\n{}", body);
    Ok(())
}

fn _build_response(status: u16, body: String) -> Result<WagiResponse<String>, Error> {
    Ok(WagiResponse(Response::builder()
        .status(status)
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .header(CONTENT_LENGTH, HeaderValue::from_str(&body.as_bytes().len().to_string())?)
        .body(body)?))
}
