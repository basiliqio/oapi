use super::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaString {
    pattern: Option<String>, //TODO Support regex-
    min_length: Option<u64>,
    max_length: Option<u64>,
    format: Option<OApiStringFormat>,
    #[serde(rename = "enum")]
    enum_: Option<Vec<String>>,
    nullable: Option<OperatorSelector<bool>>,
    read_only: Option<OperatorSelector<bool>>,
    write_only: Option<OperatorSelector<bool>>,
    example: Option<OperatorSelector<Value>>,
    deprecated: Option<OperatorSelector<bool>>,
    discriminator: Option<OperatorSelector<OApiDiscriminator>>,
    external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
