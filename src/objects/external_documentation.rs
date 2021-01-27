use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExternalDocumentation {
    url: Url,
    #[serde(default)]
    description: Option<String>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
