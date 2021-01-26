use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowAuthorizationCode<OAuthFlowAuthorizationCodeExt> {
    authorization_url: Url,
    token_url: Url,
    refresh_url: Option<Url>,
    #[serde(default)]
    scopes: HashMap<String, String>,
    #[serde(flatten)]
    extension: OAuthFlowAuthorizationCodeExt,
}
