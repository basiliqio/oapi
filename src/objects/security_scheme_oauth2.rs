use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeOauth2<
    SecuritySchemeOauth2Ext,
    SecuritySchemeOauth2FlowExt,
    SecuritySchemeOauth2FlowImplicitExt,
    SecuritySchemeOauth2FlowPasswordExt,
    SecuritySchemeOauth2FlowClientCredentialsExt,
    SecuritySchemeOauth2FlowAuthorizationCodeExt,
> {
    description: Option<String>,
    flows: OApiOAuthFlow<
        SecuritySchemeOauth2FlowExt,
        SecuritySchemeOauth2FlowImplicitExt,
        SecuritySchemeOauth2FlowPasswordExt,
        SecuritySchemeOauth2FlowClientCredentialsExt,
        SecuritySchemeOauth2FlowAuthorizationCodeExt,
    >,
    #[serde(flatten)]
    extension: SecuritySchemeOauth2Ext,
}
