//! requests - HTTP client library with simple API.\
//! If you have used Python requests module you will find the API familiar.
//!
//! # Quick Start
//!
//! ```rust
//! extern crate hyper;
//! extern crate requests;
//! let response = requests::get("http://httpbin.org/get").unwrap();
//! assert_eq!(response.url(), "http://httpbin.org/get");
//! assert_eq!(response.reason(), "OK");
//! assert_eq!(response.status_code(), hyper::Ok);
//! let data = response.json().unwrap();
//! assert_eq!(data["url"], "http://httpbin.org/get");
//! assert_eq!(data["headers"]["Host"], "httpbin.org");
//! assert_eq!(data["headers"]["User-Agent"],
//!            concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
//! ```

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
