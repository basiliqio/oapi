use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Default, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiPathItem {
    summary: Option<String>,
    description: Option<String>,
    #[serde(default)]
    get: Option<OApiOperation>,
    #[serde(default)]
    put: Option<OApiOperation>,
    #[serde(default)]
    post: Option<OApiOperation>,
    #[serde(default)]
    delete: Option<OApiOperation>,
    #[serde(default)]
    options: Option<OApiOperation>,
    #[serde(default)]
    patch: Option<OApiOperation>,
    #[serde(default)]
    head: Option<OApiOperation>,
    #[serde(default)]
    trace: Option<OApiOperation>,
    #[serde(default)]
    servers: Vec<OApiServer>,
    #[serde(default)]
    parameters: Vec<OApiParameter>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
