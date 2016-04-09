use hyper::client;
use super::response::Response;
use super::Result;

pub type Request<T> = client::request::Request<T>;

pub fn get(url: &str) -> Result<Response> {
    client::Client::new().get(url).send().map(Response::from)
}

pub fn post(url: &str) -> Result<Response> {
    client::Client::new().post(url).send().map(Response::from)
}

pub fn put(url: &str) -> Result<Response> {
    client::Client::new().put(url).send().map(Response::from)
}

pub fn head(url: &str) -> Result<Response> {
    client::Client::new().head(url).send().map(Response::from)
}

pub fn delete(url: &str) -> Result<Response> {
    client::Client::new().delete(url).send().map(Response::from)
}
