use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeApiKey {
    description: Option<String>,
    name: String,
    #[serde(rename = "in")]
    in_: OApiApiKeyLocation,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
