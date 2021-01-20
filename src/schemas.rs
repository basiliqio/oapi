use super::*;
use std::collections::HashMap;
use url::Url;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiRoot {
    openapi: String,
    info: Option<OApiInfo>,
    servers: Option<OApiServer>,
    path: HashMap<String, OApiPathItem>,
    components: Option<OApiComponents>,
    security: HashMap<String, OApiSecurityScheme>,
    tags: Vec<OApiTag>,
    external_docs: Option<OApiExternalDocumentation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiInfo {
    title: String,
    description: Option<String>,
    terms_of_services: Option<String>,
    contact: Option<OApiContact>,
    license: Option<OApiLicense>,
    version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiContact {
    name: Option<String>,
    url: Option<Url>,
    email: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLicense {
    name: String,
    url: Option<Url>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServer {
    url: Url,
    description: Option<String>,
    variables: HashMap<String, OApiServerVariable>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServerVariable {
    #[serde(rename = "enum")]
    enum_: Vec<String>,
    default: String,
    description: Option<String>,
}

type OApiCallback = HashMap<String, OApiPathItem>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiComponents {
    schemas: HashMap<String, String>, //FIXME JSONSCHEMA
    responses: HashMap<String, OApiResponse>,
    parameters: HashMap<String, OApiParameter>,
    examples: HashMap<String, OApiExampleSelector>,
    request_bodies: HashMap<String, OApiRequestBody>,
    headers: HashMap<String, OApiHeader>,
    security_schemes: HashMap<String, OApiSecurityScheme>,
    links: HashMap<String, OApiLink>,
    callbacks: HashMap<String, OApiCallback>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiPathItem {
    summary: Option<String>,
    description: Option<String>,
    get: Option<OApiOperation>,
    put: Option<OApiOperation>,
    post: Option<OApiOperation>,
    delete: Option<OApiOperation>,
    options: Option<OApiOperation>,
    patch: Option<OApiOperation>,
    head: Option<OApiOperation>,
    trace: Option<OApiOperation>,
    servers: Vec<OApiServer>,
    parameters: Vec<OApiParameter>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOperation {
    tags: Vec<String>,
    summary: Option<String>,
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation>,
    operation_id: Option<String>,
    parameters: Option<Vec<OApiParameter>>,
    request_body: Option<OApiRequestBody>,
    responses: HashMap<String, OApiResponse>,
    callbacks: HashMap<String, OApiCallback>,
    deprecated: bool,
    security: HashMap<String, Vec<String>>,
    servers: Vec<OApiServer>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExternalDocumentation {
    url: Url,
    description: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OApiParameterLocation {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OApiParameterStyle {
    Form,
    Simple,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OApiExampleSelector {
    Single(String), //FIXME VALUE
    Multiple(Vec<OApiExample>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiParameter {
    name: String,
    #[serde(rename = "in")]
    in_: OApiParameterLocation,
    description: Option<String>,
    required: bool,
    deprecated: bool,
    allow_empty_value: bool,
    style: Option<OApiParameterStyle>,
    explode: Option<bool>,
    allow_reserved: Option<bool>,
    schema: Option<String>, //FIXME JSONSCHEMA
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiRequestBody {
    description: Option<String>,
    content: HashMap<String, OApiMediaType>, //FIXME JSONSCHEMA
    required: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiMediaType {
    schema: Option<String>, //FIXME JSONSCHEMA
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
    encoding: HashMap<String, OApiEncoding>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiEncoding {
    content_type: Option<String>,
    headers: HashMap<String, OApiHeader>,
    style: Option<String>,
    explode: bool,
    allow_reserved: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiResponse {
    description: String,
    headers: HashMap<String, OApiHeader>,
    content: HashMap<String, OApiMediaType>,
    links: HashMap<String, OApiLink>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExample {
    summary: Option<String>,
    description: Option<String>,
    value: Option<String>, //FIXME Value
    external_value: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLink {
    operation_ref: Option<String>,
    operation_id: Option<String>,
    parameters: HashMap<String, String>,
    request_body: Option<String>,
    description: Option<String>,
    server: Option<OApiServer>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiHeader {
    description: Option<String>,
    required: bool,
    deprecated: bool,
    allow_empty_value: bool,
    style: Option<OApiParameterStyle>,
    explode: Option<bool>,
    allow_reserved: Option<bool>,
    schema: Option<String>, //FIXME JSONSCHEMA
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiTag {
    name: String,
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDiscriminator {
    property_name: String,
    mapping: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OApiSecurityScheme {
    ApiKey(OApiSecuritySchemeApiKey),
    Http(OApiSecuritySchemeHttp),
    Oauth2(Box<OApiSecurityOauth2>), // Boxed to reduce the size of the enum
    OpenIdConnect(OApiSecurityOpenIdConnect),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OApiApiKeyLocation {
    Query,
    Header,
    Cookie,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeApiKey {
    description: Option<String>,
    name: String,
    #[serde(rename = "in")]
    in_: OApiApiKeyLocation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeHttp {
    description: Option<String>,
    scheme: String,
    bearer_format: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecurityOauth2 {
    description: Option<String>,
    flows: OApiOAuthFlow,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecurityOpenIdConnect {
    description: Option<String>,
    #[serde(rename = "openIdConnectUrl")]
    openid_connect_url: Url,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlow {
    implicit: Option<OApiOAuthFlowImplicit>,
    password: Option<OApiOAuthFlowPassword>,
    client_credentials: Option<OApiOAuthFlowClientCredentials>,
    authorization_code: Option<OApiOAuthFlowAuthorizationCode>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowImplicit {
    authorization_url: Url,
    refresh_url: Option<String>,
    scopes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowPassword {
    token_url: Url,
    refresh_url: Option<Url>,
    scopes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowClientCredentials {
    token_url: Url,
    refresh_url: Option<Url>,
    scopes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowAuthorizationCode {
    authorization_url: Url,
    token_url: Url,
    refresh_url: Option<Url>,
    scopes: HashMap<String, String>,
}
