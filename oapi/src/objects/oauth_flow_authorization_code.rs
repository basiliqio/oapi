use super::*;

/// ## The OAuth Authorization Code Flow parameters
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowAuthorizationCode {
    authorization_url: String,
    token_url: String,
    refresh_url: Option<String>,
    #[serde(default)]
    scopes: HashMap<String, String>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
