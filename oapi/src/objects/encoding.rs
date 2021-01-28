use super::*;

/// ## The OpenApi [encoding object](https://swagger.io/specification/#encoding-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiEncoding {
    /// The content type of the property
    content_type: Option<String>,
    /// Additional information headers
    #[serde(default)]
    headers: HashMap<String, OApiHeader>,
    /// Style of the property value
    style: Option<String>,
    #[serde(default)]
    /// Mark this property as exploded
    explode: bool,
    /// Allow reserved characters in this property
    #[serde(default)]
    allow_reserved: bool,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
