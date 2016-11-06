extern crate hyper;
extern crate json;

mod request;
mod response;

pub use request::Request;
pub use response::Response;
pub use response::Codes;

pub type Result = hyper::Result<Response>;
pub type Error = hyper::error::Error;

pub fn get(url: &str) -> Result {
    Request::default().get(url)
}

pub fn post(url: &str) -> Result {
    Request::default().post(url)
}

pub fn put(url: &str) -> Result {
    Request::default().put(url)
}

pub fn head(url: &str) -> Result {
    Request::default().head(url)
}

pub fn delete(url: &str) -> Result {
    Request::default().delete(url)
}
