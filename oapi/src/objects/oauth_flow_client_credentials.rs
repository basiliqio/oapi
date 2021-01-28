use super::*;

/// ## The OAuth Client Credentials Flow parameters
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowClientCredentials {
    token_url: String,
    refresh_url: Option<String>,
    #[serde(default)]
    scopes: HashMap<String, String>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
