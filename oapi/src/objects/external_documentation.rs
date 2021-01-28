use super::*;

/// ## The OpenApi [external documentation object](https://swagger.io/specification/#external-documentation-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExternalDocumentation {
    /// The url or part of url of the documentation
    url: String,
    #[serde(default)]
    description: Option<String>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
