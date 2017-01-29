#[cfg(feature = "with_json")]
use json;
#[cfg(feature = "with_serde")]
use serde_json;

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

#[cfg(feature = "with_serde")]
impl ToJson for response::Response {
    type ResultType = serde_json::Result<serde_json::Value>;
    fn json(&self) -> <response::Response as ToJson>::ResultType {
        let text = self.text().unwrap();
        serde_json::from_str(text)
    }
}
