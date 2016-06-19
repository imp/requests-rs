extern crate hyper;
extern crate json;

pub mod request;
pub mod response;

pub use request::{get, post, put, head, delete};
pub use response::Response;

pub type RequestsResult = hyper::Result<Response>;

#[cfg(test)]
mod test {
    use super::*;
    use hyper;

    #[test]
    fn simple_get() {
        const URL: &'static str = "http://httpbin.org/get";
        let res = get(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
        let data = res.json().unwrap();
        println!("{:?}", data);
        assert!(data["url"].is(URL));
        assert!(data["headers"]["Host"].is("httpbin.org"));
        assert!(data["headers"]["User-Agent"]
            .is(concat!("requests-rs/", env!("CARGO_PKG_VERSION"))));
    }

    #[test]
    fn simple_post() {
        const URL: &'static str = "http://httpbin.org/post";
        let res = post(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn simple_put() {
        const URL: &'static str = "http://httpbin.org/put";
        let res = put(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn simple_head() {
        const URL: &'static str = "http://httpbin.org/get";
        let res = head(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn simple_delete() {
        const URL: &'static str = "http://httpbin.org/delete";
        let res = delete(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn user_agent() {
        let useragent = concat!("{\n  \"user-agent\": \"requests-rs/",
                                env!("CARGO_PKG_VERSION"),
                                "\"\n}\n");
        const URL: &'static str = "http://httpbin.org/user-agent";
        let res = get(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
        assert_eq!(res.text(), Some(useragent));
    }

    #[test]
    fn user_agent_json() {

        const URL: &'static str = "http://httpbin.org/user-agent";
        let res = get(URL).unwrap();
        assert!(res.is_json());

        let ua = res.json().unwrap();
        assert!(ua["user-agent"].is(concat!("requests-rs/", env!("CARGO_PKG_VERSION"))));
    }
}
