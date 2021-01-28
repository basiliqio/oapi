use super::*;

/// ## An object in a schema
#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaObject {
    /// A list of required properties for this object
    required: Vec<String>,
    /// A map to the properties of this object
    properties: Option<OperatorSelector<HashMap<String, OperatorSelector<OApiSchema>>>>,
    /// The upper limit of properties number
    max_properties: Option<OperatorSelector<u64>>,
    /// The lower limit of properties number
    min_properties: Option<OperatorSelector<u64>>,
    /// Mark this object as nullable
    nullable: Option<OperatorSelector<bool>>,
    /// Mark this object as read-only
    read_only: Option<OperatorSelector<bool>>,
    /// Mark this object as write-only
    write_only: Option<OperatorSelector<bool>>,
    /// An example to illustrate this object
    example: Option<OperatorSelector<Value>>,
    /// Mark this object as deprecated
    deprecated: Option<OperatorSelector<bool>>,
    /// A discriminator for this object
    discriminator: Option<OperatorSelector<OApiSchemaDiscriminator>>,
    /// External documentation for this object, if any
    external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
