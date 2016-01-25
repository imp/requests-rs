mod request;
mod response;

use hyper;

pub type Result<T> = hyper::error::Result<T>;

pub use self::request::{get, post};
