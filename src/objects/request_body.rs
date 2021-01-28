use super::*;

/// ## The OpenApi [request body object](https://swagger.io/specification/#request-body-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiRequestBody {
    /// Description of the request body
    #[serde(default)]
    description: Option<String>,
    /// Content of the request body
    #[serde(default)]
    content: HashMap<String, OApiMediaType>, // TODO Check that key is a valid media type
    #[serde(default)]
    /// Is the request body required ?
    required: bool,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
