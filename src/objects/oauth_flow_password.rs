use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowPassword {
    token_url: Url,
    refresh_url: Option<Url>,
    #[serde(default)]
    scopes: HashMap<String, String>,
}
