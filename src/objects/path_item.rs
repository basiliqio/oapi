use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiPathItem {
    summary: Option<String>,
    description: Option<String>,
    get: Option<OApiOperation>,
    put: Option<OApiOperation>,
    post: Option<OApiOperation>,
    delete: Option<OApiOperation>,
    options: Option<OApiOperation>,
    patch: Option<OApiOperation>,
    head: Option<OApiOperation>,
    trace: Option<OApiOperation>,
    #[serde(default)]
    servers: Vec<OApiServer>,
    #[serde(default)]
    parameters: Vec<OApiParameter>,
}
