use super::*;

/// ## The OAuth Implicit Flow parameters
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowImplicit {
    authorization_url: String,
    refresh_url: Option<String>,
    #[serde(default)]
    scopes: HashMap<String, String>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
