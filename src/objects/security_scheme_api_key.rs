use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeApiKey {
    description: Option<String>,
    name: String,
    #[serde(rename = "in")]
    in_: OApiApiKeyLocation,
}
