use super::*;

/// ## An object to switch between multiple schemas depending on a field value
#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaDiscriminator {
    /// The 'type' property to use as discriminator
    property_name: String,
    /// The mapping between `property_name` values and schema to use
    mapping: OperatorSelector<HashMap<String, SparseRefRawInline<OApiSchema>>>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
