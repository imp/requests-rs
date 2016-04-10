mod request;
mod response;

use hyper;

pub type Result<T> = hyper::Result<T>;

pub use self::request::*;
