use super::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaArray {
    additional_items: Option<OApiSchemaAdditionalItem>,
    max_items: Option<u64>,
    min_items: Option<u64>,
    items: Option<OperatorSelector<OApiSchema>>,
    unique_items: bool,
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
