extern crate hyper;
extern crate requests;
#[cfg(feature = "with_json")]
extern crate json;

// use requests::Codes;
use requests::{delete, get, head, post, put};
use requests::{Codes, Request, Response, StatusCode};
use requests::ToJson;

fn assert_response_is_ok(response: &Response, url: &str) {
    assert_eq!(response.url(), url);
    assert_eq!(response.status_code(), StatusCode::Ok);
    assert_eq!(response.reason(), "OK");
}

#[test]
fn simple_get() {
    const URL: &'static str = "http://httpbin.org/get";
    let res = get(URL).unwrap();
    assert_response_is_ok(&res, URL);
    let data = res.json().unwrap();
    println!("{:?}", data);
    assert_eq!(data["url"], URL);
    assert_eq!(data["headers"]["Host"], "httpbin.org");
    assert_eq!(data["headers"]["User-Agent"],
               concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
}

#[test]
fn simple_get_string_url_https() {
    const URL: &'static str = "https://httpbin.org/get";
    let res = get(String::from(URL)).unwrap();
    assert_response_is_ok(&res, URL);
    let res = get(&String::from(URL)).unwrap();
    assert_response_is_ok(&res, URL);
}

#[test]
fn simple_get_string_url() {
    const URL: &'static str = "http://httpbin.org/get";
    let res = get(String::from(URL)).unwrap();
    assert_response_is_ok(&res, URL);
    let res = get(&String::from(URL)).unwrap();
    assert_response_is_ok(&res, URL);
}

#[test]
fn simple_post() {
    const URL: &'static str = "http://httpbin.org/post";
    let res = post(URL).unwrap();
    assert_response_is_ok(&res, URL);
}

#[test]
fn simple_put() {
    const URL: &'static str = "http://httpbin.org/put";
    let res = put(URL).unwrap();
    assert_response_is_ok(&res, URL);
}

#[test]
fn simple_head() {
    const URL: &'static str = "http://httpbin.org/get";
    let res = head(URL).unwrap();
    assert_response_is_ok(&res, URL);
}

#[test]
fn simple_delete() {
    const URL: &'static str = "http://httpbin.org/delete";
    let res = delete(URL).unwrap();
    assert_response_is_ok(&res, URL);
}

#[test]
fn user_agent() {
    let useragent = concat!("{\n  \"user-agent\": \"requests-rs/",
                            env!("CARGO_PKG_VERSION"),
                            "\"\n}\n");
    const URL: &'static str = "http://httpbin.org/user-agent";
    let res = get(URL).unwrap();
    assert_response_is_ok(&res, URL);
    assert_eq!(res.text(), Some(useragent));
}

#[test]
fn custom_user_agent() {
    const UA: &'static str = concat!("requests-rs-tests/", env!("CARGO_PKG_VERSION"));
    const URL: &'static str = "http://httpbin.org/user-agent";
    let mut request = Request::new();
    request.user_agent(UA);
    let res = request.get(URL).unwrap();
    assert_response_is_ok(&res, URL);
    assert!(res.is_json());

    let ua = res.json().unwrap();
    assert_eq!(ua["user-agent"], UA);
}

#[test]
fn user_agent_json() {

    const URL: &'static str = "http://httpbin.org/user-agent";
    let res = get(URL).unwrap();
    assert_response_is_ok(&res, URL);
    assert!(res.is_json());

    let ua = res.json().unwrap();
    assert_eq!(ua["user-agent"],
               concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
}

#[test]
fn content() {
    const URL: &'static str = "http://httpbin.org/headers";
    let content = concat!("{\n  \"headers\": {\n    \"Host\": \"httpbin.org\",",
                          " \n    \"User-Agent\": \"requests-rs/",
                          env!("CARGO_PKG_VERSION"),
                          "\"\n  }\n}\n");

    let res = get(URL).unwrap();
    assert_response_is_ok(&res, URL);
    assert_eq!(res.content(), &content.as_bytes());
}

#[test]
fn headers() {
    use hyper::header::UserAgent;

    const URL: &'static str = "http://httpbin.org/response-headers?User-Agent=requests-rs-test";
    let res = get(URL).unwrap();
    assert_response_is_ok(&res, URL);
    println!("{:?}", res.text());
    println!("{:?}", res.headers());
    println!("{:?}", res.headers().get::<UserAgent>());
    assert_eq!(res.headers().get::<UserAgent>().unwrap(),
               &UserAgent("requests-rs-test".to_owned()));
}

#[cfg(feature = "with_json")]
#[test]
fn accept_json() {
    const URL: &'static str = "http://httpbin.org/headers";
    let res = Request::json().get(URL).unwrap();
    assert_response_is_ok(&res, URL);
    assert!(res.is_json());
    let data = res.json().unwrap();
    println!("{:?}", data);
    assert_eq!(data["headers"]["Accept"], "application/json");
}

macro_rules! status_code_test {
    ($($name:ident: $numeric:expr,)+) => {
        $(#[test]
        fn $name() {
            let res = get(&format!("http://httpbin.org/status/{}", $numeric)).unwrap();
            println!("{}", res.text().unwrap());
            assert_eq!(res.status_code(), Codes::from_u16($numeric));
        })+
    }
}

status_code_test! {
    status_code_100: 100,
    status_code_101: 101,
    status_code_102: 102,
    status_code_200: 200,
    status_code_201: 201,
    status_code_202: 202,
    status_code_203: 203,
    status_code_204: 204,
    status_code_205: 205,
    status_code_206: 206,
    status_code_207: 207,
    status_code_208: 208,
    status_code_226: 226,
    status_code_300: 300,
    // status_code_301: 301,
    // status_code_302: 302,
    // status_code_303: 303,
    status_code_304: 304,
    // status_code_305: 305,
    // status_code_307: 307,
    status_code_308: 308,
    status_code_400: 400,
    status_code_401: 401,
    status_code_402: 402,
    status_code_403: 403,
    status_code_404: 404,
    status_code_405: 405,
    status_code_406: 406,
    status_code_407: 407,
    status_code_408: 408,
    status_code_409: 409,
    status_code_410: 410,
    status_code_411: 411,
    status_code_412: 412,
    status_code_413: 413,
    status_code_414: 414,
    status_code_415: 415,
    status_code_416: 416,
    status_code_417: 417,
    status_code_418: 418,
    status_code_419: 419,
    status_code_420: 420,
    status_code_421: 421,
    status_code_422: 422,
    status_code_423: 423,
    status_code_424: 424,
    status_code_426: 426,
    status_code_428: 428,
    status_code_429: 429,
    status_code_431: 431,
    status_code_451: 451,
    status_code_500: 500,
    status_code_501: 501,
    status_code_502: 502,
    status_code_503: 503,
    status_code_504: 504,
    status_code_505: 505,
    status_code_506: 506,
    status_code_507: 507,
    status_code_508: 508,
    status_code_510: 510,
    status_code_511: 511,
}
