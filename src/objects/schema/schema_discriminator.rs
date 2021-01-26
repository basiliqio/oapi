use super::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default, OApiCheck,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaDiscriminator<DiscriminatorExt> {
    property_name: String,
    mapping: OperatorSelector<HashMap<String, String>>,
    #[serde(flatten)]
    extension: DiscriminatorExt,
}
