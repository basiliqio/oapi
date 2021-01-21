use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLink {
    operation_ref: Option<String>,
    operation_id: Option<String>,
    #[serde(default)]
    parameters: HashMap<String, String>,
    request_body: Option<String>,
    description: Option<String>,
    server: Option<OApiServer>,
}
