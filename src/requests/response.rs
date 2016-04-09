use std::io::Read;
use std::convert::From;
use hyper;

pub type HyperResponse = hyper::client::response::Response;

#[derive(Debug)]
pub struct Response(hyper::client::response::Response);

impl From<hyper::client::response::Response> for Response {
    fn from(raw: hyper::client::response::Response) -> Self {
        Response(raw)
    }
}

impl Response {
    pub fn url(&self) -> String {
        self.0.url.serialize()
    }

    pub fn status_code(&self) -> hyper::status::StatusCode {
        self.0.status
    }

    pub fn reason(&self) -> String {
        self.0.status.canonical_reason().unwrap_or("UNAVAILABLE").to_owned()
    }

    pub fn ok(&self) -> bool {
        self.0.status == hyper::status::StatusCode::Ok
    }

    pub fn text(&mut self) -> String {
        let mut text = String::new();
        self.0.read_to_string(&mut text);
        text
    }

    pub fn json(&self) -> bool {
        unimplemented!();
    }
}
