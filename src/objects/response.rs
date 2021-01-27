use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiResponse {
    description: String,
    #[serde(default)]
    headers: HashMap<String, OApiHeader>,
    #[serde(default)]
    content: HashMap<String, OApiMediaType>,
    #[serde(default)]
    links: HashMap<String, OApiLink>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
