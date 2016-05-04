use std::io::Read;
use std::convert::From;
use std::str;
use hyper::client;
use hyper::header::{ContentLength, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use serde::Deserialize;
use serde_json;

pub type HyperResponse = client::Response;

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
    pub fn url(&self) -> &str {
        self.hr.url.as_str()
    }

    pub fn status_code(&self) -> StatusCode {
        self.hr.status
    }

    pub fn reason(&self) -> &str {
        self.hr.status.canonical_reason().unwrap_or("UNAVAILABLE")
    }

    pub fn ok(&self) -> bool {
        self.hr.status == StatusCode::Ok
    }

    pub fn text(&'a self) -> Option<&'a str> {
        str::from_utf8(&self.content).ok()
    }

    pub fn json(&self) -> Option<String> {
        if self.is_json() {
            self.text().map(|x| x.to_owned())
        } else {
            None
        }
    }

    pub fn from_json<T>(&self) -> Option<T> where T: Deserialize {
        self.text().and_then(|t| serde_json::from_str::<T>(t).ok())
    }

    pub fn is_json(&self) -> bool {
        match self.hr.headers.get::<ContentType>() {
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Json, _))) => true,
            _ => false,
        }
    }
}
