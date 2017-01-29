//! requests - HTTP client library with simple API.\
//! If you have used Python requests module you will find the API familiar.
//!
//! # Quick Start
//!
//! ```rust
//! extern crate requests;
//! use requests::ToJson;
//!
//! fn main() {
//!     let response = requests::get("http://httpbin.org/get").unwrap();
//!     assert_eq!(response.url(), "http://httpbin.org/get");
//!     assert_eq!(response.reason(), "OK");
//!     assert_eq!(response.status_code(), requests::StatusCode::Ok);
//!
//!     let data = response.json().unwrap();
//!     assert_eq!(data["url"], "http://httpbin.org/get");
//!     assert_eq!(data["headers"]["Host"], "httpbin.org");
//!     assert_eq!(data["headers"]["User-Agent"],
//!                concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
//! }
//! ```

extern crate hyper;
#[cfg(feature = "ssl")]
extern crate hyper_native_tls;
#[cfg(feature = "with_json")]
extern crate json;
#[cfg(feature = "with_serde")]
extern crate serde_json;

mod request;
mod response;
mod tojson;

pub use request::Request;
pub use response::Response;
pub use response::{Codes, StatusCode};
pub use tojson::ToJson;

pub type Result = hyper::Result<Response>;
pub type Error = hyper::error::Error;

pub fn get<T: AsRef<str>>(url: T) -> Result {
    Request::default().get(url.as_ref())
}

pub fn post<T: AsRef<str>>(url: T) -> Result {
    Request::default().post(url.as_ref())
}

pub fn put<T: AsRef<str>>(url: T) -> Result {
    Request::default().put(url.as_ref())
}

pub fn head<T: AsRef<str>>(url: T) -> Result {
    Request::default().head(url.as_ref())
}

pub fn delete<T: AsRef<str>>(url: T) -> Result {
    Request::default().delete(url.as_ref())
}
