use super::*;

/// ## The OpenApi [contact object](https://swagger.io/specification/#contact-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiContact {
    /// Name of the contact
    name: Option<String>,
    /// Url of the contact
    url: Option<String>,
    /// Email of the contact
    email: Option<String>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
