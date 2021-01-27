use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiEncoding {
    content_type: Option<String>,
    #[serde(default)]
    headers: HashMap<String, OApiHeader>,
    style: Option<String>,
    #[serde(default)]
    explode: bool,
    #[serde(default)]
    allow_reserved: bool,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
