use super::*;

/// ## An string in a schema
#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaString {
    /// A regex this string should support
    pattern: Option<String>, //TODO Support regex-
    /// The lower bound for the string length
    min_length: Option<u64>,
    /// The upper bound for the string length
    max_length: Option<u64>,
    /// The format to use for the string
    format: Option<OApiStringFormat>,
    /// An enum of valid values
    #[serde(rename = "enum")]
    enum_: Option<Vec<String>>,
    /// Mark the string as nullable
    nullable: Option<OperatorSelector<bool>>,
    /// Mark the string as read-only
    read_only: Option<OperatorSelector<bool>>,
    /// Mark the string as write-only
    write_only: Option<OperatorSelector<bool>>,
    /// An example for the string
    example: Option<OperatorSelector<Value>>,
    /// Mark this string as deprecated
    deprecated: Option<OperatorSelector<bool>>,
    /// A discriminator for this string
    discriminator: Option<OperatorSelector<OApiSchemaDiscriminator>>,
    /// External documentation for this string, if any
    external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
