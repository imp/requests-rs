extern crate hyper;

pub mod requests;

#[cfg(test)]
mod test {
    use super::requests;
    use hyper;

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
        const URL: &'static str = "http://httpbin.org/user-agent";
        let res = requests::get(URL).unwrap();
        println!("{:?}", res.text());
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
        assert_eq!(res.text(), Some("{\n  \"user-agent\": \"requests-rs/0.0.0\"\n}\n"));
    }
}
