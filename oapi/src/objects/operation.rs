use super::*;

/// ## The OpenApi [operation object](https://swagger.io/specification/#operation-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOperation {
    /// The tags this operation corresponds to
    #[serde(default)]
    tags: Vec<String>,
    /// A summary of the operation
    #[serde(default)]
    summary: Option<String>,
    /// A description of the operation
    #[serde(default)]
    description: Option<String>,
    /// An external documentation for this operation
    external_docs: Option<OApiExternalDocumentation>,
    /// An unique operation id
    #[serde(default)]
    operation_id: Option<String>,
    /// Parameters for the operations
    #[serde(default)]
    parameters: Vec<OApiParameter>,
    /// A request body object
    request_body: Option<OApiRequestBody>,
    /// A map of responses
    #[serde(default)]
    responses: HashMap<String, OApiResponse>,
    /// A map of callbacks
    #[serde(default)]
    callbacks: HashMap<String, OApiCallback>,
    /// Flag marking an operation as deprecated
    #[serde(default)]
    deprecated: bool,
    /// A list of security to use
    #[serde(default)]
    security: Vec<HashMap<String, Vec<String>>>,
    /// A list of servers to use
    #[serde(default)]
    servers: Vec<OApiServer>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
