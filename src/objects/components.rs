use super::*;

/// ## The OpenApi [components](https://swagger.io/specification/#components-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiComponents {
    /// A map of reusable links
    links: HashMap<String, OApiLink>,
    /// A map of reusable schemas
    schemas: HashMap<String, OperatorSelector<OApiSchema>>,
    /// A map of reusable responses
    responses: HashMap<String, OApiResponse>,
    /// A map of reusable parameters
    parameters: HashMap<String, OApiParameter>,
    /// A map of reusable examples
    examples: HashMap<String, OApiExampleSelector>,
    /// A map of reusable request bodies
    request_bodies: HashMap<String, OApiRequestBody>,
    /// A map of reusable headers
    headers: HashMap<String, OApiHeader>,
    /// A map of reusable security schemes
    security_schemes: HashMap<String, OApiSecurityScheme>,
    /// A map of reusable callbacks
    callbacks: HashMap<String, OApiCallback>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
