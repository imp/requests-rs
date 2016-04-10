use hyper::client;
use hyper::header::UserAgent;
use super::response::Response;
use super::Result;

pub type Request<T> = client::request::Request<T>;

const REQUESTS_USER_AGENT: &'static str = "requests-rs/0.0.0";

pub fn get(url: &str) -> Result<Response> {
    client::Client::new()
        .get(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}

pub fn post(url: &str) -> Result<Response> {
    client::Client::new()
        .post(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}

pub fn put(url: &str) -> Result<Response> {
    client::Client::new()
        .put(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}

pub fn head(url: &str) -> Result<Response> {
    client::Client::new()
        .head(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}

pub fn delete(url: &str) -> Result<Response> {
    client::Client::new()
        .delete(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}
