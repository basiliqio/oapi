use super::*;

/// ## A numeric object in a schema
#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaNumeric {
    /// Define this number should be a multiple of
    multiple_of: Option<Option<u64>>,
    /// Upper bound for this number
    #[serde(flatten)]
    maximum: Option<OApiNumericMaximum>,
    /// Lower bound for this number
    #[serde(flatten)]
    minimum: Option<OApiNumericMinimum>,
    /// Format to use for this number
    format: Option<OApiNumericFormat>,
    /// Mark this number as nullable
    nullable: Option<OperatorSelector<bool>>,
    /// Mark this number as read_only
    read_only: Option<OperatorSelector<bool>>,
    /// Mark this number as write_only
    write_only: Option<OperatorSelector<bool>>,
    /// An example for this number
    example: Option<OperatorSelector<Value>>,
    /// Mark this number as deprecated
    deprecated: Option<OperatorSelector<bool>>,
    /// A discriminator for this number
    discriminator: Option<OperatorSelector<OApiSchemaDiscriminator>>,
    /// An external documentation for this number, if any
    external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
