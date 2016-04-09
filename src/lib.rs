extern crate hyper;

pub mod requests;

#[cfg(test)]
mod test {
    use super::requests;
    use hyper::status::StatusCode;

    #[test]
    fn test_get() {
        const URL: &'static str = "http://httpbin.org/get";
        let res = requests::get(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), StatusCode::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn test_post() {
        const URL: &'static str = "http://httpbin.org/post";
        let res = requests::post(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), StatusCode::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn test_put() {
        const URL: &'static str = "http://httpbin.org/put";
        let res = requests::put(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), StatusCode::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn test_head() {
        const URL: &'static str = "http://httpbin.org/get";
        let res = requests::head(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), StatusCode::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn test_detele() {
        const URL: &'static str = "http://httpbin.org/delete";
        let res = requests::delete(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), StatusCode::Ok);
        assert_eq!(res.reason(), "OK");
    }
}
