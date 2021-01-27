use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeHttp {
    description: Option<String>,
    scheme: String,
    bearer_format: Option<String>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
