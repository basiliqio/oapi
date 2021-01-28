use super::*;

/// ## The OpenApi [response object](https://swagger.io/specification/#response-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiResponse {
    /// A description of the response
    description: String,
    /// Defined headers of the response
    #[serde(default)]
    headers: HashMap<String, OApiHeader>,
    /// A map of media type for the responses
    #[serde(default)]
    content: HashMap<String, OApiMediaType>,
    /// Links for the responses
    #[serde(default)]
    links: HashMap<String, OApiLink>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
