use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default, OApiCheck,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiComponents {
    links: HashMap<String, OApiLink>,
    schemas: HashMap<String, OperatorSelector<OApiSchema>>,
    responses: HashMap<String, OApiResponse>,
    parameters: HashMap<String, OApiParameter>,
    examples: HashMap<String, OApiExampleSelector>,
    request_bodies: HashMap<String, OApiRequestBody>,
    headers: HashMap<String, OApiHeader>,
    security_schemes: HashMap<String, OApiSecurityScheme>,
    callbacks: HashMap<String, OApiCallback>,
}
