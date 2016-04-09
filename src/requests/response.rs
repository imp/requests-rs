use std::io::Read;
use std::convert::From;
use std::str;
use hyper::client::response;
use hyper::header::ContentLength;
use hyper::status::StatusCode;

pub type HyperResponse = response::Response;

#[derive(Debug)]
pub struct Response {
    content: Vec<u8>,
    hr: HyperResponse,
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
            hr: raw,
        }
    }
}

impl<'a> Response {
    pub fn url(&self) -> String {
        self.hr.url.serialize()
    }

    pub fn status_code(&self) -> StatusCode {
        self.hr.status
    }

    pub fn reason(&self) -> String {
        self.hr.status.canonical_reason().unwrap_or("UNAVAILABLE").to_owned()
    }

    pub fn ok(&self) -> bool {
        self.hr.status == StatusCode::Ok
    }

    pub fn text(&'a self) -> Option<&'a str> {
        str::from_utf8(&self.content).ok()
    }

    pub fn json(&self) -> bool {
        unimplemented!();
    }
}
