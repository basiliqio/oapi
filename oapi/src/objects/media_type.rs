use super::*;

/// ## The OpenApi [media type object](https://swagger.io/specification/#media-type-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Default, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiMediaType {
    /// The schema defining this object for this media type
    #[serde(default)]
    schema: Option<OperatorSelector<OApiSchema>>,
    /// An example or examples to illustrate this media type schema
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
    /// A map of the encodings to use
    #[serde(default)]
    encoding: HashMap<String, OApiEncoding>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
