use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDiscriminator {
    property_name: String,
    #[serde(default)]
    mapping: HashMap<String, String>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
