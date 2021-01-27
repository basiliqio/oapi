use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default, OApiCheck,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiComponents {
    #[serde(default)]
    links: HashMap<String, OApiLink>,
    schemas: HashMap<String, OperatorSelector<OApiSchema>>,
    #[serde(default)]
    responses: HashMap<String, OApiResponse>,
    parameters: HashMap<String, OApiParameter>,
    #[serde(default)]
    examples: HashMap<String, OApiExampleSelector>,
    request_bodies: HashMap<String, OApiRequestBody>,
    headers: HashMap<String, OApiHeader>,
    security_schemes: HashMap<String, OApiSecurityScheme>,
    // callbacks: HashMap<String, OApiCallback>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
