use super::*;

/// ## An array object in a schema
#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaArray {
    /// Additional items, if any
    additional_items: Option<OApiSchemaAdditionalItem>,
    /// The max number of items in the array
    max_items: Option<u64>,
    /// The min number of items in the array
    min_items: Option<u64>,
    /// A description of the array's items
    items: Option<OperatorSelector<OApiSchema>>,
    /// Are items to be unique in the array ?
    unique_items: bool,
    /// Is the array nullable ?
    nullable: Option<OperatorSelector<bool>>,
    /// Mark this array as read-only data
    read_only: Option<OperatorSelector<bool>>,
    /// Mark this array as write-only data
    write_only: Option<OperatorSelector<bool>>,
    /// An example of this array
    example: Option<OperatorSelector<Value>>,
    /// Mark this array deprecated
    deprecated: Option<OperatorSelector<bool>>,
    /// A discriminator to switch between multiple types of arrays
    discriminator: Option<OperatorSelector<OApiSchemaDiscriminator>>,
    /// External documentation, if any
    external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
