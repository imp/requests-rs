use hyper::client::{Client, IntoUrl};
use hyper::header::{Headers, Accept, UserAgent};
use hyper::net::HttpsConnector;
use hyper::Url;
#[cfg(feature = "ssl")]
use hyper_native_tls::NativeTlsClient;

use super::Response;
use super::Result;

const DEFAULT_USER_AGENT: &'static str = concat!("requests-rs/", env!("CARGO_PKG_VERSION"));


fn get_hyper_client(url: &Url) -> Client {
    if url.scheme() == "https" {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        Client::with_connector(connector)
    } else {
        Client::new()
    }

}

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
        self.headers.set(UserAgent(ua.to_owned()))
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> Result {
        let url = url.into_url()?;
        get_hyper_client(&url)
            .get(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> Result {
        let url = url.into_url()?;
        get_hyper_client(&url)
            .post(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn put<U: IntoUrl>(&self, url: U) -> Result {
        let url = url.into_url()?;
        get_hyper_client(&url)
            .put(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn head<U: IntoUrl>(&self, url: U) -> Result {
        let url = url.into_url()?;
        get_hyper_client(&url)
            .head(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }

    pub fn delete<U: IntoUrl>(&self, url: U) -> Result {
        let url = url.into_url()?;
        get_hyper_client(&url)
            .delete(url)
            .headers(self.headers.clone())
            .send()
            .map(Response::from)
    }
}
