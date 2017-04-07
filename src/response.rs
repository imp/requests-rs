use std::io::Read;
use std::convert::From;
use std::str;

use reqwest;
use reqwest::header::{Headers, ContentLength, ContentType};
use serde::de::DeserializeOwned;
use serde_json;

pub use reqwest::StatusCode;
pub type Codes = StatusCode;
pub type ReqwestResponse = reqwest::Response;

#[derive(Debug)]
pub struct Response {
    content: Vec<u8>,
    inner: ReqwestResponse,
}

impl From<ReqwestResponse> for Response {
    fn from(mut raw: ReqwestResponse) -> Self {
        let mut content = match raw.headers().get::<ContentLength>() {
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
        self.inner.url().as_str()
    }

    pub fn status_code(&self) -> Codes {
        self.inner.status()
    }

    pub fn reason(&self) -> &str {
        self.inner
            .status()
            .canonical_reason()
            .unwrap_or("UNAVAILABLE")
    }

    pub fn ok(&self) -> bool {
        self.inner.status() == StatusCode::Ok
    }

    pub fn text(&'a self) -> Option<&'a str> {
        str::from_utf8(&self.content).ok()
    }

    pub fn content(&'a self) -> &'a Vec<u8> {
        &self.content
    }

    pub fn is_json(&self) -> bool {
        self.inner.headers().get::<ContentType>() == Some(&ContentType::json())
    }

    pub fn headers(&self) -> &Headers {
        self.inner.headers()
    }

    pub fn json<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        //self.inner.json()
        let text = self.text().unwrap();
        serde_json::from_str(text)
    }
}
