use std::io::Read;
use std::convert::From;
use std::str;
use hyper;
use hyper::header::{Headers, ContentLength, ContentType};

pub use hyper::status::StatusCode;
pub type Codes = StatusCode;
pub type HyperResponse = hyper::client::Response;

#[derive(Debug)]
pub struct Response {
    content: Vec<u8>,
    inner: HyperResponse,
}

impl From<HyperResponse> for Response {
    fn from(mut raw: HyperResponse) -> Self {
        let mut content = match raw.headers.get::<ContentLength>() {
            Some(&ContentLength(length)) => Vec::with_capacity(length as usize),
            None => Vec::new(),
        };

        if raw.read_to_end(&mut content).is_err() {
            content = Vec::new()
        }

        Response {
            content: content,
            inner: raw,
        }
    }
}

impl<'a> Response {
    pub fn url(&self) -> &str {
        self.inner.url.as_str()
    }

    pub fn status_code(&self) -> Codes {
        self.inner.status
    }

    pub fn reason(&self) -> &str {
        self.inner.status.canonical_reason().unwrap_or("UNAVAILABLE")
    }

    pub fn ok(&self) -> bool {
        self.inner.status == StatusCode::Ok
    }

    pub fn text(&'a self) -> Option<&'a str> {
        str::from_utf8(&self.content).ok()
    }

    pub fn content(&'a self) -> &'a Vec<u8> {
        &self.content
    }

    pub fn is_json(&self) -> bool {
        self.inner.headers.get::<ContentType>() == Some(&ContentType::json())
    }

    pub fn headers(&self) -> &Headers {
        &self.inner.headers
    }
}
