use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeOpenIdConnect {
    description: Option<String>,
    #[serde(rename = "openIdConnectUrl")]
    openid_connect_url: Url,
}
