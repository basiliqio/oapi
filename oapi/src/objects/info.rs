use super::*;

/// ## The OpenApi document [information object](https://swagger.io/specification/#info-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiInfo {
    /// Title of the document
    title: String,
    /// Description of the document
    description: Option<String>,
    /// Terms of services, if any
    terms_of_services: Option<String>,
    /// Persons of contacts for this document
    contact: Option<OApiContact>,
    /// License of this document
    license: Option<OApiLicense>,
    /// The revision of this document
    version: String,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
