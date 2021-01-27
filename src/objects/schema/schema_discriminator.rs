use super::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaDiscriminator {
    property_name: String,
    mapping: OperatorSelector<HashMap<String, String>>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
