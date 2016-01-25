use hyper;

pub type HyperResponse = hyper::client::response::Response;

#[derive(Debug)]
pub struct Response {
    raw: HyperResponse,
    pub url: String,
    pub status_code: u16,
    pub reason: String,
}

impl Response {
    pub fn from_hyper(raw: HyperResponse) -> Self {
        let status_code;
        let reason;
        {
            let &hyper::http::RawStatus(sc, ref r) = raw.status_raw();
            status_code = sc;
            reason = r.to_string();
        }
        let url = raw.url.serialize();
        Response {
            raw: raw,
            url: url,
            status_code: status_code,
            reason: reason,
        }
    }
}
