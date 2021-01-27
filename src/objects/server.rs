use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServer {
    url: String,
    description: Option<String>,
    #[serde(default)]
    variables: HashMap<String, OApiServerVariable>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
