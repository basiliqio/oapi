use super::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaArray {
    additional_items: OApiSchemaAdditionalItem,
    max_items: Option<u64>,
    min_items: Option<u64>,
    items: Option<OperatorSelector<OApiSchema>>,
    #[serde(default)]
    unique_items: bool,
    #[serde(flatten)]
    common: OApiSchemaCommon,
}
