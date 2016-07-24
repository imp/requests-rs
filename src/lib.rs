extern crate hyper;
extern crate json;

pub mod request;
pub mod response;

pub use request::{get, post, put, head, delete};
pub use response::Response;

pub type Result = hyper::Result<Response>;
pub type Error = hyper::error::Error;
