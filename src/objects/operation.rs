use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOperation {
    #[serde(default)]
    tags: Vec<String>,
    summary: Option<String>,
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation>,
    operation_id: Option<String>,
    parameters: Option<Vec<OApiParameter>>,
    request_body: Option<OApiRequestBody>,
    #[serde(default)]
    responses: HashMap<String, OApiResponse>,
    #[serde(default)]
    callbacks: HashMap<String, OApiCallback>,
    deprecated: bool,
    #[serde(default)]
    security: HashMap<String, Vec<String>>,
    servers: Vec<OApiServer>,
}
