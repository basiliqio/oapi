use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDocument {
    openapi: String,
    info: OApiInfo,
    servers: Option<Vec<OApiServer>>,
    #[serde(default)]
    path: HashMap<String, OApiPathItem>,
    components: Option<OApiComponents>,
    #[serde(default)]
    security: HashMap<String, OApiSecurityScheme>,
    tags: Option<Vec<OApiTag>>,
    external_docs: Option<OApiExternalDocumentation>,
}
