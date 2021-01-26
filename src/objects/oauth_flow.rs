use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlow<
    OAuthFlowExt,
    OAuthFlowImplicitExt,
    OAuthFlowPasswordExt,
    OAuthFlowClientCredentialsExt,
    OAuthFlowAuthorizationCodeExt,
> {
    implicit: Option<OApiOAuthFlowImplicit<OAuthFlowImplicitExt>>,
    password: Option<OApiOAuthFlowPassword<OAuthFlowPasswordExt>>,
    client_credentials: Option<OApiOAuthFlowClientCredentials<OAuthFlowClientCredentialsExt>>,
    authorization_code: Option<OApiOAuthFlowAuthorizationCode<OAuthFlowAuthorizationCodeExt>>,
    #[serde(flatten)]
    extension: OAuthFlowExt,
}
