use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServerVariable {
    #[serde(default)]
    #[serde(rename = "enum")]
    enum_: Vec<String>,
    default: String,
    description: Option<String>,
}
