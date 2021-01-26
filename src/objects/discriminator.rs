use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDiscriminator<DiscriminatorExt> {
    property_name: String,
    #[serde(default)]
    mapping: HashMap<String, String>,
    #[serde(flatten)]
    extension: DiscriminatorExt,
}
