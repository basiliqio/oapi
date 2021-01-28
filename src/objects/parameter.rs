use super::*;

/// ## The OpenApi [parameter object](https://swagger.io/specification/#parameter-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiParameter {
    /// Name of the parameter
    name: String,
    /// Position of the parameter
    #[serde(rename = "in")]
    in_: OApiParameterLocation,
    /// Description of the parameter
    description: Option<String>,
    /// Mark the parameter as mandatory
    required: Option<bool>,
    /// Mark the parameter as deprecated
    #[serde(default)]
    deprecated: bool,
    /// Mark if the parameter should allow empty values
    #[serde(default)]
    allow_empty_value: bool,
    /// Style of the parameter
    style: Option<OApiParameterStyle>,
    /// Mark if the parameter should be exploded
    #[serde(default)]
    explode: bool,
    /// Mark if the paramter should allow reserved characters
    #[serde(default)]
    allow_reserved: bool,
    /// Schema description the parameter
    schema: Option<OperatorSelector<OApiSchema>>,
    /// Example object for the parameter
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
