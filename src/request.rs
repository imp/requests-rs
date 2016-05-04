use hyper;
use hyper::header::UserAgent;
use super::response::Response;
use super::RequestsResult;

const DEFAULT_USER_AGENT: &'static str = concat!("requests-rs/", env!("CARGO_PKG_VERSION"));

fn default_user_agent() -> UserAgent {
    UserAgent(DEFAULT_USER_AGENT.to_owned())
}

pub fn get(url: &str) -> RequestsResult {
    hyper::Client::new()
        .get(url)
        .header(default_user_agent())
        .send()
        .map(Response::from)
}

pub fn post(url: &str) -> RequestsResult {
    hyper::Client::new()
        .post(url)
        .header(default_user_agent())
        .send()
        .map(Response::from)
}

pub fn put(url: &str) -> RequestsResult {
    hyper::Client::new()
        .put(url)
        .header(default_user_agent())
        .send()
        .map(Response::from)
}

pub fn head(url: &str) -> RequestsResult {
    hyper::Client::new()
        .head(url)
        .header(default_user_agent())
        .send()
        .map(Response::from)
}

pub fn delete(url: &str) -> RequestsResult {
    hyper::Client::new()
        .delete(url)
        .header(default_user_agent())
        .send()
        .map(Response::from)
}
