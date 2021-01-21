use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExample {
    summary: Option<String>,
    description: Option<String>,
    value: Option<Value>,
    external_value: Option<String>,
}
