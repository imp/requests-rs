use hyper;
use hyper::client::IntoUrl;
use hyper::header::{Headers, UserAgent};
use super::Response;
use super::Result;

const DEFAULT_USER_AGENT: &'static str = concat!("requests-rs/", env!("CARGO_PKG_VERSION"));

fn default_user_agent() -> UserAgent {
    UserAgent(DEFAULT_USER_AGENT.to_owned())
}

pub struct Request {
    headers: Headers,
}

impl Default for Request {
    fn default() -> Self {
        let mut headers = Headers::new();
        headers.set(default_user_agent());
        Request { headers: headers }
    }
}

impl Request {
    pub fn get<U: IntoUrl>(&self, url: U) -> Result {
        hyper::Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> Result {
        hyper::Client::new()
            .post(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn put<U: IntoUrl>(&self, url: U) -> Result {
        hyper::Client::new()
            .put(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn head<U: IntoUrl>(&self, url: U) -> Result {
        hyper::Client::new()
            .head(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn delete<U: IntoUrl>(&self, url: U) -> Result {
        hyper::Client::new()
            .delete(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }
}
