use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowImplicit {
    authorization_url: Url,
    refresh_url: Option<String>,
    #[serde(default)]
    scopes: HashMap<String, String>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
