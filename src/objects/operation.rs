use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOperation {
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation>,
    #[serde(default)]
    operation_id: Option<String>,
    #[serde(default)]
    parameters: Vec<OApiParameter>,
    request_body: Option<OApiRequestBody>,
    #[serde(default)]
    responses: HashMap<String, OApiResponse>,
    #[serde(default)]
    callbacks: HashMap<String, OApiCallback>,
    #[serde(default)]
    deprecated: bool,
    #[serde(default)]
    security: Vec<HashMap<String, Vec<String>>>,
    #[serde(default)]
    servers: Vec<OApiServer>,
}
