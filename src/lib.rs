extern crate hyper;
extern crate json;

pub mod request;
pub mod response;

pub use request::{get, post, put, head, delete};
pub use response::Response;

pub type RequestsResult = hyper::Result<Response>;
pub type RequestsError = hyper::error::Error;
