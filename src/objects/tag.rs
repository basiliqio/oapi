use super::*;

/// ## The OpenApi [tag object](https://swagger.io/specification/#tag-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiTag {
    /// The name of the tag
    name: String,
    /// A description of the tag
    #[serde(default)]
    description: Option<String>,
    /// External documentation for the tag
    #[serde(default)]
    external_docs: Option<OApiExternalDocumentation>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
