#[cfg(feature = "with_json")]
use json;

use response;

pub trait ToJson {
    type ResultType;
    fn json(&self) -> Self::ResultType;
}

#[cfg(feature = "with_json")]
impl ToJson for response::Response {
    type ResultType = json::Result<json::JsonValue>;
    fn json(&self) -> <response::Response as ToJson>::ResultType {
        let text = self.text().unwrap();
        json::parse(text)
        // self.text().map(|t| json::parse(t)).unwrap()
    }
}
