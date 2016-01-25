extern crate hyper;

pub mod requests;

#[cfg(test)]
mod test {
    use super::requests;
    use hyper;

    #[test]
    fn test_get() {
        let res = requests::get("http://httpbin.org/get").unwrap();
        assert_eq!(res.status, hyper::Ok);
    }

    #[test]
    fn test_post() {
        let res = requests::post("http://httpbin.org/post").unwrap();
        assert_eq!(res.status, hyper::Ok);
    }
}
