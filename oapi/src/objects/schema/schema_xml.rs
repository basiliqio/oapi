use super::*;

/// ## The OpenApi [xml object](https://swagger.io/specification/#xml-object)
#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaXml {
    name: Option<String>,
    namespace: Option<String>,
    prefix: Option<String>,
    attribute: Option<bool>,
    wrapped: Option<bool>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
