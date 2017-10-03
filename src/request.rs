use reqwest;
use reqwest::IntoUrl;
use reqwest::header::{Headers, Accept, UserAgent};

use super::Response;
use super::Result;

const DEFAULT_USER_AGENT: &'static str = concat!("requests-rs/", env!("CARGO_PKG_VERSION"));

#[derive(Debug)]
pub struct Request {
    headers: Headers,
}

impl Default for Request {
    fn default() -> Self {
        let mut request = Request::new();
        request.user_agent(DEFAULT_USER_AGENT);
        request
    }
}

impl Request {
    pub fn new() -> Self {
        Request { headers: Headers::new() }
    }

    pub fn json() -> Self {
        let mut request = Request::new();
        request.user_agent(DEFAULT_USER_AGENT);
        request.headers.set(Accept::json());
        request
    }

    pub fn user_agent(&mut self, ua: &str) {
        self.headers.set(UserAgent::new(ua.to_owned()))
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> Result {
        reqwest::Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> Result {
        reqwest::Client::new()
            .post(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn put<U: IntoUrl>(&self, url: U) -> Result {
        reqwest::Client::new()
            .put(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn head<U: IntoUrl>(&self, url: U) -> Result {
        reqwest::Client::new()
            .head(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn delete<U: IntoUrl>(&self, url: U) -> Result {
        reqwest::Client::new()
            .delete(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }
}
