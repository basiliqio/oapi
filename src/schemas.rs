use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDocument {
    openapi: String,
    info: OApiInfo,
    servers: Option<Vec<OApiServer>>,
    #[serde(default)]
    path: HashMap<String, OApiPathItem>,
    components: Option<OApiComponents>,
    #[serde(default)]
    security: HashMap<String, OApiSecurityScheme>,
    tags: Option<Vec<OApiTag>>,
    external_docs: Option<OApiExternalDocumentation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiContact {
    name: Option<String>,
    url: Option<Url>,
    email: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLicense {
    name: String,
    url: Option<Url>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServer {
    url: String,
    description: Option<String>,
    #[serde(default)]
    variables: HashMap<String, OApiServerVariable>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServerVariable {
    #[serde(default)]
    #[serde(rename = "enum")]
    enum_: Vec<String>,
    default: String,
    description: Option<String>,
}

type OApiCallback = HashMap<String, OApiPathItem>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiComponents {
    schemas: HashMap<String, OApiSchema>,
    responses: HashMap<String, OApiResponse>,
    parameters: HashMap<String, OApiParameter>,
    examples: HashMap<String, OApiExampleSelector>,
    // request_bodies: HashMap<String, OApiRequestBody>>>,
    headers: HashMap<String, OApiHeader>,
    security_schemes: HashMap<String, OApiSecurityScheme>,
    links: HashMap<String, OApiLink>,
    callbacks: HashMap<String, OApiCallback>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
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
    #[serde(default)]
    servers: Vec<OApiServer>,
    #[serde(default)]
    parameters: Vec<OApiParameter>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOperation {
    #[serde(default)]
    tags: Vec<String>,
    summary: Option<String>,
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation>,
    operation_id: Option<String>,
    parameters: Option<Vec<OApiParameter>>,
    request_body: Option<OApiRequestBody>,
    #[serde(default)]
    responses: HashMap<String, OApiResponse>,
    #[serde(default)]
    callbacks: HashMap<String, OApiCallback>,
    deprecated: bool,
    #[serde(default)]
    security: HashMap<String, Vec<String>>,
    servers: Vec<OApiServer>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExternalDocumentation {
    url: Url,
    description: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiParameterLocation {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiParameterStyle {
    Form,
    Simple,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiExampleSelector {
    Single(OApiExample),
    Multiple(Vec<OApiExample>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
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
    schema: Option<OApiSchema>,
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiRequestBody {
    description: Option<String>,
    content: HashMap<String, OApiMediaType>,
    required: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiMediaType {
    schema: Option<OApiSchema>,
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
    #[serde(default)]
    encoding: HashMap<String, OApiEncoding>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiEncoding {
    content_type: Option<String>,
    #[serde(default)]
    headers: HashMap<String, OApiHeader>,
    style: Option<String>,
    #[serde(default)]
    explode: bool,
    #[serde(default)]
    allow_reserved: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiResponse {
    description: String,
    #[serde(default)]
    headers: HashMap<String, OApiHeader>,
    #[serde(default)]
    content: HashMap<String, OApiMediaType>,
    #[serde(default)]
    links: HashMap<String, OApiLink>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExample {
    summary: Option<String>,
    description: Option<String>,
    value: Option<Value>,
    external_value: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLink {
    operation_ref: Option<String>,
    operation_id: Option<String>,
    #[serde(default)]
    parameters: HashMap<String, String>,
    request_body: Option<String>,
    description: Option<String>,
    server: Option<OApiServer>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
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
    schema: Option<OApiSchema>,
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiTag {
    name: String,
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDiscriminator {
    property_name: String,
    #[serde(default)]
    mapping: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OApiSecurityScheme {
    ApiKey(OApiSecuritySchemeApiKey),
    Http(OApiSecuritySchemeHttp),
    Oauth2(Box<OApiSecurityOauth2>), // Boxed to reduce the size of the enum
    OpenIdConnect(OApiSecurityOpenIdConnect),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiApiKeyLocation {
    Query,
    Header,
    Cookie,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeApiKey {
    description: Option<String>,
    name: String,
    #[serde(rename = "in")]
    in_: OApiApiKeyLocation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeHttp {
    description: Option<String>,
    scheme: String,
    bearer_format: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecurityOauth2 {
    description: Option<String>,
    flows: OApiOAuthFlow,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecurityOpenIdConnect {
    description: Option<String>,
    #[serde(rename = "openIdConnectUrl")]
    openid_connect_url: Url,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlow {
    implicit: Option<OApiOAuthFlowImplicit>,
    password: Option<OApiOAuthFlowPassword>,
    client_credentials: Option<OApiOAuthFlowClientCredentials>,
    authorization_code: Option<OApiOAuthFlowAuthorizationCode>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowImplicit {
    authorization_url: Url,
    refresh_url: Option<String>,
    #[serde(default)]
    scopes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowPassword {
    token_url: Url,
    refresh_url: Option<Url>,
    #[serde(default)]
    scopes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowClientCredentials {
    token_url: Url,
    refresh_url: Option<Url>,
    #[serde(default)]
    scopes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowAuthorizationCode {
    authorization_url: Url,
    token_url: Url,
    refresh_url: Option<Url>,
    #[serde(default)]
    scopes: HashMap<String, String>,
}
