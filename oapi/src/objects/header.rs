use super::*;

/// ## The OpenApi [header object](https://swagger.io/specification/#header-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiHeader {
    /// A description of the header object
    description: Option<String>,
    /// Mark the header as required
    required: bool,
    /// Mark the header as deprecated
    deprecated: bool,
    /// Mark the header as accepting empty values
    allow_empty_value: bool,
    /// Describe the header style
    style: Option<OApiParameterStyle>,
    /// Mark the header as exploded
    explode: bool,
    /// Mark the header as accepting reserved characters
    allow_reserved: bool,
    /// Describe the schema of the header
    schema: Option<OperatorSelector<OApiSchema>>,
    /// An example of the header to illustrate its schema
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
