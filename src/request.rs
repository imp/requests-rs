use hyper;
use hyper::client::IntoUrl;
use hyper::header::{Headers, UserAgent};
use super::Response;
use super::Result;

const DEFAULT_USER_AGENT: &'static str = concat!("requests-rs/", env!("CARGO_PKG_VERSION"));

pub struct Request {
    headers: Headers,
    params: Vec<String>,
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
        Request {
            headers: Headers::new(),
            params: Vec::<String>::new(),
        }
    }

    pub fn params<T, U>(&mut self, params: U) -> &mut Self
        where T: Into<String>,
              U: IntoIterator<Item = (T, T)>
    {
        for (key, value) in params {
            self.param(key, value);
        }
        self
    }

    pub fn param<T>(&mut self, key: T, value: T) -> &mut Self
        where T: Into<String>
    {
        self.params.push(format!("{}={}", key.into(), value.into()));
        self
    }

    pub fn user_agent(&mut self, ua: &str) {
        self.headers.set(UserAgent(ua.to_owned()))
    }

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
