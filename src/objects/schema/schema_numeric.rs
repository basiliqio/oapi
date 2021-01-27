use super::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaNumeric {
    multiple_of: Option<Option<u64>>,
    #[serde(flatten)]
    maximum: Option<OApiNumericMaximum>,
    #[serde(flatten)]
    minimum: Option<OApiNumericMinimum>,
    format: Option<OApiNumericFormat>,
    nullable: Option<OperatorSelector<bool>>,
    read_only: Option<OperatorSelector<bool>>,
    write_only: Option<OperatorSelector<bool>>,
    example: Option<OperatorSelector<Value>>,
    deprecated: Option<OperatorSelector<bool>>,
    discriminator: Option<OperatorSelector<OApiDiscriminator>>,
    external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
