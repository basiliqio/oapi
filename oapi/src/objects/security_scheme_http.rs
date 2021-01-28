use super::*;

/// ## Parameter for HTTP security scheme
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeHttp {
    /// Description of the http auth
    description: Option<String>,
    /// Scheme to use for this auth
    scheme: String,
    /// The bearer format string
    bearer_format: Option<String>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
