use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable, OApiCheck)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OApiSecurityScheme {
    ApiKey(OApiSecuritySchemeApiKey),
    Http(OApiSecuritySchemeHttp),
    Oauth2(Box<OApiSecuritySchemeOauth2>), // Boxed to reduce the size of the enum
    OpenIdConnect(OApiSecuritySchemeOpenIdConnect),
}
