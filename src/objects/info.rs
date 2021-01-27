use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiInfo {
    title: String,
    description: Option<String>,
    terms_of_services: Option<String>,
    contact: Option<OApiContact>,
    license: Option<OApiLicense>,
    version: String,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
