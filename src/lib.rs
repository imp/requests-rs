extern crate hyper;

pub mod requests;

#[cfg(test)]
mod test {
    use super::requests;

    #[test]
    fn test_get() {
        let res = requests::get("http://httpbin.org/get").unwrap();
        assert_eq!(res.status_code, 200);
        assert_eq!(res.reason, "OK");
    }

    #[test]
    fn test_post() {
        let res = requests::post("http://httpbin.org/post").unwrap();
        assert_eq!(res.status_code, 200);
        assert_eq!(res.reason, "OK");
    }

    #[test]
    fn test_put() {
        let res = requests::put("http://httpbin.org/put").unwrap();
        assert_eq!(res.status_code, 200);
        assert_eq!(res.reason, "OK");
    }

    #[test]
    fn test_head() {
        let res = requests::head("http://httpbin.org/get").unwrap();
        assert_eq!(res.status_code, 200);
        assert_eq!(res.reason, "OK");
    }

    #[test]
    fn test_detele() {
        let res = requests::delete("http://httpbin.org/delete").unwrap();
        assert_eq!(res.status_code, 200);
        assert_eq!(res.reason, "OK");
    }
}
