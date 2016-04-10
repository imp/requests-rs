use hyper;
use hyper::header::UserAgent;
use super::response::Response;
use super::Result;

const REQUESTS_USER_AGENT: &'static str = "requests-rs/0.0.0";

pub fn get(url: &str) -> Result<Response> {
    hyper::Client::new()
        .get(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}

pub fn post(url: &str) -> Result<Response> {
    hyper::Client::new()
        .post(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}

pub fn put(url: &str) -> Result<Response> {
    hyper::Client::new()
        .put(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}

pub fn head(url: &str) -> Result<Response> {
    hyper::Client::new()
        .head(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}

pub fn delete(url: &str) -> Result<Response> {
    hyper::Client::new()
        .delete(url)
        .header(UserAgent(REQUESTS_USER_AGENT.to_owned()))
        .send()
        .map(Response::from)
}
