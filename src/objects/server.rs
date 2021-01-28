use super::*;

/// ## The OpenApi [server object](https://swagger.io/specification/#server-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServer {
    /// Url or portion of url of the server
    url: String,
    /// A description of the server
    description: Option<String>,
    /// A map to the server variables
    #[serde(default)]
    variables: HashMap<String, OApiServerVariable>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
