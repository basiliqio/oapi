use super::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, OApiCheck, Default,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaObject {
    required: Vec<String>,
    properties: Option<OperatorSelector<HashMap<String, OperatorSelector<OApiSchema>>>>,
    max_properties: Option<OperatorSelector<u64>>,
    min_properties: Option<OperatorSelector<u64>>,
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
