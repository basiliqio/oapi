use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaObject {
    required: Vec<String>,
    properties: OperatorSelector<HashMap<String, OperatorSelector<OApiSchema>>>,
    max_properties: Option<OperatorSelector<u64>>,
    min_properties: Option<OperatorSelector<u64>>,
    #[serde(flatten)]
    common: OApiSchemaCommon,
}
