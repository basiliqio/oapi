use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDiscriminator {
    property_name: String,
    #[serde(default)]
    mapping: HashMap<String, String>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
