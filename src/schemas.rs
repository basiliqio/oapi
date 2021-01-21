use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDocument {
	openapi: OperatorSelector<String>,
	info: OperatorSelector<OApiInfo>,
	servers: Option<OperatorSelector<Vec<OperatorSelector<OApiServer>>>>,
	#[serde(default)]
	path: OperatorSelector<HashMap<String, OperatorSelector<OApiPathItem>>>,
	components: Option<OperatorSelector<OApiComponents>>,
	#[serde(default)]
	security: OperatorSelector<HashMap<String, OperatorSelector<OApiSecurityScheme>>>,
	tags: Option<OperatorSelector<Vec<OperatorSelector<OApiTag>>>>,
	external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiInfo {
	title: OperatorSelector<String>,
	description: Option<OperatorSelector<String>>,
	terms_of_services: Option<OperatorSelector<String>>,
	contact: Option<OperatorSelector<OApiContact>>,
	license: Option<OperatorSelector<OApiLicense>>,
	version: OperatorSelector<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiContact {
	name: Option<OperatorSelector<String>>,
	url: Option<OperatorSelector<Url>>,
	email: Option<OperatorSelector<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLicense {
	name: OperatorSelector<String>,
	url: Option<OperatorSelector<Url>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServer {
	url: OperatorSelector<String>,
	description: Option<OperatorSelector<String>>,
	#[serde(default)]
	variables: HashMap<String, OperatorSelector<OApiServerVariable>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServerVariable {
	#[serde(default)]
	#[serde(rename = "enum")]
	enum_: OperatorSelector<Vec<OperatorSelector<String>>>,
	default: OperatorSelector<String>,
	description: Option<OperatorSelector<String>>,
}

type OApiCallback = HashMap<String, OApiPathItem>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiComponents {
	schemas: OperatorSelector<HashMap<String, OperatorSelector<OApiSchema>>>,
	responses: OperatorSelector<HashMap<String, OperatorSelector<OApiResponse>>>,
	parameters: OperatorSelector<HashMap<String, OperatorSelector<OApiParameter>>>,
	examples: OperatorSelector<HashMap<String, OperatorSelector<OApiExampleSelector>>>,
	// request_bodies: OperatorSelector<HashMap<String, OperatorSelector<OApiRequestBody>>>,
	headers: OperatorSelector<HashMap<String, OperatorSelector<OApiHeader>>>,
	security_schemes: OperatorSelector<HashMap<String, OperatorSelector<OApiSecurityScheme>>>,
	links: OperatorSelector<HashMap<String, OperatorSelector<OApiLink>>>,
	callbacks: OperatorSelector<HashMap<String, OperatorSelector<OApiCallback>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiPathItem {
	summary: Option<OperatorSelector<String>>,
	description: Option<OperatorSelector<String>>,
	get: Option<OperatorSelector<OApiOperation>>,
	put: Option<OperatorSelector<OApiOperation>>,
	post: Option<OperatorSelector<OApiOperation>>,
	delete: Option<OperatorSelector<OApiOperation>>,
	options: Option<OperatorSelector<OApiOperation>>,
	patch: Option<OperatorSelector<OApiOperation>>,
	head: Option<OperatorSelector<OApiOperation>>,
	trace: Option<OperatorSelector<OApiOperation>>,
	#[serde(default)]
	servers: OperatorSelector<Vec<OperatorSelector<OApiServer>>>,
	#[serde(default)]
	parameters: OperatorSelector<Vec<OperatorSelector<OApiParameter>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOperation {
	#[serde(default)]
	tags: OperatorSelector<Vec<OperatorSelector<String>>>,
	summary: Option<OperatorSelector<String>>,
	description: Option<OperatorSelector<String>>,
	external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
	operation_id: Option<OperatorSelector<String>>,
	parameters: Option<OperatorSelector<Vec<OperatorSelector<OApiParameter>>>>,
	request_body: Option<OperatorSelector<OApiRequestBody>>,
	#[serde(default)]
	responses: OperatorSelector<HashMap<String, OperatorSelector<OApiResponse>>>,
	#[serde(default)]
	callbacks: OperatorSelector<HashMap<String, OperatorSelector<OApiCallback>>>,
	deprecated: OperatorSelector<bool>,
	#[serde(default)]
	security: OperatorSelector<HashMap<String, OperatorSelector<Vec<OperatorSelector<String>>>>>,
	servers: OperatorSelector<Vec<OperatorSelector<OApiServer>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExternalDocumentation {
	url: OperatorSelector<Url>,
	description: Option<OperatorSelector<String>>,
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
	Single(OperatorSelector<OApiExample>),
	Multiple(OperatorSelector<Vec<OperatorSelector<OApiExample>>>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiParameter {
	name: OperatorSelector<String>,
	#[serde(rename = "in")]
	in_: OperatorSelector<OApiParameterLocation>,
	description: Option<OperatorSelector<String>>,
	required: OperatorSelector<bool>,
	deprecated: OperatorSelector<bool>,
	allow_empty_value: OperatorSelector<bool>,
	style: Option<OperatorSelector<OApiParameterStyle>>,
	explode: Option<OperatorSelector<bool>>,
	allow_reserved: Option<OperatorSelector<bool>>,
	schema: Option<OperatorSelector<OApiSchema>>,
	#[serde(flatten)]
	example: Option<OperatorSelector<OApiExampleSelector>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiRequestBody {
	description: Option<OperatorSelector<String>>,
	content: OperatorSelector<HashMap<String, OperatorSelector<OApiMediaType>>>,
	required: OperatorSelector<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiMediaType {
	schema: Option<OperatorSelector<OApiSchema>>,
	#[serde(flatten)]
	example: Option<OperatorSelector<OApiExampleSelector>>,
	#[serde(default)]
	encoding: OperatorSelector<HashMap<String, OperatorSelector<OApiEncoding>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiEncoding {
	content_type: Option<OperatorSelector<String>>,
	#[serde(default)]
	headers: OperatorSelector<HashMap<String, OperatorSelector<OApiHeader>>>,
	style: Option<OperatorSelector<String>>,
	#[serde(default)]
	explode: OperatorSelector<bool>,
	#[serde(default)]
	allow_reserved: OperatorSelector<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiResponse {
	description: OperatorSelector<String>,
	#[serde(default)]
	headers: OperatorSelector<HashMap<String, OperatorSelector<OApiHeader>>>,
	#[serde(default)]
	content: OperatorSelector<HashMap<String, OperatorSelector<OApiMediaType>>>,
	#[serde(default)]
	links: OperatorSelector<HashMap<String, OperatorSelector<OApiLink>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExample {
	summary: Option<OperatorSelector<String>>,
	description: Option<OperatorSelector<String>>,
	value: Option<OperatorSelector<Value>>,
	external_value: Option<OperatorSelector<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLink {
	operation_ref: Option<OperatorSelector<String>>,
	operation_id: Option<OperatorSelector<String>>,
	#[serde(default)]
	parameters: OperatorSelector<HashMap<String, OperatorSelector<String>>>,
	request_body: Option<OperatorSelector<String>>,
	description: Option<OperatorSelector<String>>,
	server: Option<OperatorSelector<OApiServer>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiHeader {
	description: Option<OperatorSelector<String>>,
	required: OperatorSelector<bool>,
	deprecated: OperatorSelector<bool>,
	allow_empty_value: OperatorSelector<bool>,
	style: Option<OperatorSelector<OApiParameterStyle>>,
	explode: Option<OperatorSelector<bool>>,
	allow_reserved: Option<OperatorSelector<bool>>,
	schema: Option<OperatorSelector<OApiSchema>>,
	#[serde(flatten)]
	example: Option<OperatorSelector<OApiExampleSelector>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiTag {
	name: OperatorSelector<String>,
	description: Option<OperatorSelector<String>>,
	external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDiscriminator {
	property_name: OperatorSelector<String>,
	#[serde(default)]
	mapping: OperatorSelector<HashMap<String, OperatorSelector<String>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OApiSecurityScheme {
	ApiKey(OperatorSelector<OApiSecuritySchemeApiKey>),
	Http(OperatorSelector<OApiSecuritySchemeHttp>),
	Oauth2(Box<OperatorSelector<OApiSecurityOauth2>>), // Boxed to reduce the size of the enum
	OpenIdConnect(OperatorSelector<OApiSecurityOpenIdConnect>),
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
	description: Option<OperatorSelector<String>>,
	name: OperatorSelector<String>,
	#[serde(rename = "in")]
	in_: OperatorSelector<OApiApiKeyLocation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeHttp {
	description: Option<OperatorSelector<String>>,
	scheme: OperatorSelector<String>,
	bearer_format: Option<OperatorSelector<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecurityOauth2 {
	description: Option<OperatorSelector<String>>,
	flows: OperatorSelector<OApiOAuthFlow>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecurityOpenIdConnect {
	description: Option<OperatorSelector<String>>,
	#[serde(rename = "openIdConnectUrl")]
	openid_connect_url: OperatorSelector<Url>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlow {
	implicit: Option<OperatorSelector<OApiOAuthFlowImplicit>>,
	password: Option<OperatorSelector<OApiOAuthFlowPassword>>,
	client_credentials: Option<OperatorSelector<OApiOAuthFlowClientCredentials>>,
	authorization_code: Option<OperatorSelector<OApiOAuthFlowAuthorizationCode>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowImplicit {
	authorization_url: OperatorSelector<Url>,
	refresh_url: Option<OperatorSelector<String>>,
	#[serde(default)]
	scopes: OperatorSelector<HashMap<String, OperatorSelector<String>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowPassword {
	token_url: OperatorSelector<Url>,
	refresh_url: Option<OperatorSelector<Url>>,
	#[serde(default)]
	scopes: OperatorSelector<HashMap<String, OperatorSelector<String>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowClientCredentials {
	token_url: OperatorSelector<Url>,
	refresh_url: Option<OperatorSelector<Url>>,
	#[serde(default)]
	scopes: OperatorSelector<HashMap<String, OperatorSelector<String>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOAuthFlowAuthorizationCode {
	authorization_url: OperatorSelector<Url>,
	token_url: OperatorSelector<Url>,
	refresh_url: Option<OperatorSelector<Url>>,
	#[serde(default)]
	scopes: OperatorSelector<HashMap<String, OperatorSelector<String>>>,
}
