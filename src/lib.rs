#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate hyper;

pub mod requests;

#[cfg(test)]
mod test {
    use super::requests;
    use hyper;
    use serde::Deserialize;

    #[test]
    fn get() {
        const URL: &'static str = "http://httpbin.org/get";
        let res = requests::get(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn post() {
        const URL: &'static str = "http://httpbin.org/post";
        let res = requests::post(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn put() {
        const URL: &'static str = "http://httpbin.org/put";
        let res = requests::put(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn head() {
        const URL: &'static str = "http://httpbin.org/get";
        let res = requests::head(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn delete() {
        const URL: &'static str = "http://httpbin.org/delete";
        let res = requests::delete(URL).unwrap();
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
        let res = requests::get(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
        assert_eq!(res.text(), Some(useragent));
    }

    #[test]
    fn user_agent_json() {

        #[derive(Debug, Deserialize)]
        struct UserAgent {
            #[serde(rename="user-agent")]
            user_agent: String,
        }

        const URL: &'static str = "http://httpbin.org/user-agent";
        let res = requests::get(URL).unwrap();
        assert!(res.is_json());

        let ua: UserAgent = res.from_json().unwrap();
        assert_eq!(ua.user_agent, concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
    }
}
