use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlow {
    implicit: Option<OApiOAuthFlowImplicit>,
    password: Option<OApiOAuthFlowPassword>,
    client_credentials: Option<OApiOAuthFlowClientCredentials>,
    authorization_code: Option<OApiOAuthFlowAuthorizationCode>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
