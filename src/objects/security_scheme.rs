use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OApiSecurityScheme<
    SecuritySchemeApiKeyExt,
    SecuritySchemeHttpExt,
    SecuritySchemeOauth2Ext,
    SecuritySchemeOpenIdConnectExt,
    SecuritySchemeOauth2FlowExt,
    SecuritySchemeOauth2FlowImplicitExt,
    SecuritySchemeOauth2FlowPasswordExt,
    SecuritySchemeOauth2FlowClientCredentialsExt,
    SecuritySchemeOauth2FlowAuthorizationCodeExt,
> {
    ApiKey(OApiSecuritySchemeApiKey<SecuritySchemeApiKeyExt>),
    Http(OApiSecuritySchemeHttp<SecuritySchemeHttpExt>),
    Oauth2(
        Box<
            OApiSecuritySchemeOauth2<
                SecuritySchemeOauth2Ext,
                SecuritySchemeOauth2FlowExt,
                SecuritySchemeOauth2FlowImplicitExt,
                SecuritySchemeOauth2FlowPasswordExt,
                SecuritySchemeOauth2FlowClientCredentialsExt,
                SecuritySchemeOauth2FlowAuthorizationCodeExt,
            >,
        >,
    ), // Boxed to reduce the size of the enum
    OpenIdConnect(OApiSecuritySchemeOpenIdConnect<SecuritySchemeOpenIdConnectExt>),
}
