use super::*;

/// ## Parameter for Api Key security scheme
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeApiKey {
    /// Description of the api key
    description: Option<String>,
    /// Name of the header or cookie parameter
    name: String,
    /// Location of the Api Key
    #[serde(rename = "in")]
    in_: OApiApiKeyLocation,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
