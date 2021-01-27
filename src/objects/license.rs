use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLicense {
    name: String,
    url: Option<Url>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
