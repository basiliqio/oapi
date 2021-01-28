use super::*;

/// ## The OpenApi [example object](https://swagger.io/specification/#example-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExample {
    /// A summary of the example
    summary: Option<String>,
    /// A description of the example
    description: Option<String>,
    /// The value of the example
    value: Option<Value>,
    /// An url to a distant example value for this example
    external_value: Option<String>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
