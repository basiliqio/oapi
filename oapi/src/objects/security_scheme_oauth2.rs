use super::*;

/// ## Parameter for OAuth2 security scheme
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeOauth2 {
    /// A description of the oauth2 scheme
    description: Option<String>,
    /// The flow to use
    flows: OApiOAuthFlow,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
