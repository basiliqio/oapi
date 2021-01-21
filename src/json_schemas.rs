use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
pub enum OApiNumericMaximum {
    #[serde(rename = "maximum")]
    Inclusive(u64),
    #[serde(rename = "exclusiveMaximum")]
    Exclusive(u64),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable)]
pub enum OApiNumericMinimum {
    #[serde(rename = "minimum")]
    Inclusive(u64),
    #[serde(rename = "exclusiveMinimum")]
    Exclusive(u64),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiNumericFormat {
    Int32,
    Int64,
    Float,
    Double,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiStringFormat {
    Byte,
    Binary,
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    Password,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaString {
    pattern: Option<String>, //TODO Support regex-
    min_length: Option<u64>,
    max_length: Option<u64>,
    format: Option<OApiStringFormat>,
    #[serde(rename = "enum")]
    enum_: Option<Vec<String>>,
    #[serde(flatten)]
    common: OApiSchemaCommon,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
pub enum OApiSchemaAdditionalItem {
    Any(bool),
    Obj(Box<OperatorSelector<OApiSchema>>),
}

impl std::default::Default for OApiSchemaAdditionalItem {
    fn default() -> Self {
        OApiSchemaAdditionalItem::Any(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaArray {
    additional_items: OApiSchemaAdditionalItem,
    max_items: Option<u64>,
    min_items: Option<u64>,
    items: Option<OperatorSelector<OApiSchema>>,
    #[serde(default)]
    unique_items: bool,
    #[serde(flatten)]
    common: OApiSchemaCommon,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaObject {
    required: Vec<String>,
    properties: OperatorSelector<HashMap<String, OperatorSelector<OApiSchema>>>,
    max_properties: Option<OperatorSelector<u64>>,
    min_properties: Option<OperatorSelector<u64>>,
    #[serde(flatten)]
    common: OApiSchemaCommon,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaNumeric {
    multiple_of: Option<OperatorSelector<u64>>,
    #[serde(flatten)]
    maximum: Option<OperatorSelector<OApiNumericMaximum>>,
    #[serde(flatten)]
    minimum: Option<OperatorSelector<OApiNumericMinimum>>,
    format: Option<OApiNumericFormat>,
    #[serde(flatten)]
    common: OApiSchemaCommon,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaDiscriminator {
    property_name: String,
    mapping: OperatorSelector<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaXml {
    name: Option<String>,
    namespace: Option<String>,
    prefix: Option<String>,
    attribute: Option<bool>,
    wrapped: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaCommon {
    nullable: Option<OperatorSelector<bool>>,
    read_only: Option<OperatorSelector<bool>>,
    write_only: Option<OperatorSelector<bool>>,
    example: Option<OperatorSelector<Value>>,
    deprecated: Option<OperatorSelector<bool>>,
    discriminator: Option<OperatorSelector<OApiDiscriminator>>,
    external_docs: Option<OperatorSelector<OApiExternalDocumentation>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(tag = "type")]
pub enum OApiSchema {
    #[serde(rename = "object")]
    Obj(Box<OperatorSelector<OApiSchemaObject>>),
    #[serde(rename = "array")]
    Array(Box<OperatorSelector<OApiSchemaArray>>),
    #[serde(rename = "number")]
    #[serde(alias = "integer")]
    Numeric(OperatorSelector<OApiSchemaNumeric>),
    #[serde(rename = "string")]
    String(OperatorSelector<OApiSchemaString>),
    #[serde(rename = "boolean")]
    Bool,
    #[serde(rename = "null")]
    Null,
}
