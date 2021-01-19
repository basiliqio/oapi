use super::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiRoot {
    openapi: String,
    info: Option<OApiInfo>,
    servers: Option<OApiServer>,
    path: String,          //FIXME
    components: String,    //FIXME
    security: Vec<String>, //FIXME
    tags: Vec<String>,     //FIXME
    external_docs: String, // FIXME
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
    url: Option<String>,   //TODO Parse url
    email: Option<String>, // Parse email
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLicense {
    name: String,
    url: Option<String>, //TODO Parse url
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServer {
    url: String,
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiComponents {
    schemas: HashMap<String, String>,          //FIXME
    responses: HashMap<String, String>,        //FIXME
    parameters: HashMap<String, String>,       //FIXME
    examples: HashMap<String, String>,         //FIXME
    request_bodies: HashMap<String, String>,   //FIXME
    headers: HashMap<String, String>,          //FIXME
    security_schemes: HashMap<String, String>, //FIXME
    links: HashMap<String, String>,            //FIXME
    callbacks: HashMap<String, String>,        //FIXME
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiPathItem {
    summary: Option<String>,
    description: Option<String>,
    get: Option<OApiOperation>,     //FIXME
    put: Option<OApiOperation>,     //FIXME
    post: Option<OApiOperation>,    //FIXME
    delete: Option<OApiOperation>,  //FIXME
    options: Option<OApiOperation>, //FIXME
    patch: Option<OApiOperation>,   //FIXME
    head: Option<OApiOperation>,    //FIXME
    trace: Option<OApiOperation>,   //FIXME
    servers: Vec<OApiServer>,
    parameters: String, //FIXME
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOperation {
    tags: Vec<String>,
    summary: Option<String>,
    description: Option<String>,
    external_docs: Option<String>, //FIXME
    operation_id: Option<String>,
    parameters: Option<String>,   // FIXME
    request_body: Option<String>, //FIXME
    responses: Option<String>,    //FIXME
    callbacks: Option<String>,    //FIXME
    deprecated: bool,             //FIXME
    security: Vec<String>,        //FIXME
    servers: Vec<OApiServer>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExternalDocumentation {
    url: String,
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
    schema: Option<String>,   //FIXME
    example: Option<String>,  //FIXME
    examples: Option<String>, //FIXME
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiRequestBody {
    description: Option<String>,
    content: Option<String>, //FIXME
    required: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiMediaType {
    schema: Option<String>,   //FIXME
    example: Option<String>,  // FIXME use either
    examples: Option<String>, //FIXME
    encoding: Option<String>, //FIXME
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiResponse {
    description: String,
    headers: Option<String>, //FIXME
    content: Option<String>, //FIXME
    links: Option<String>,   //FIXME
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExample {
    summary: Option<String>,
    description: Option<String>,
    value: Option<String>, //FIXME use serde_json::Value
    external_value: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLink {
    operation_ref: Option<String>,
    operation_id: Option<String>,
    parameters: HashMap<String, String>,
    request_body: Option<String>, //FIXME
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
    schema: Option<String>,   //FIXME
    example: Option<String>,  //FIXME
    examples: Option<String>, //FIXME
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
