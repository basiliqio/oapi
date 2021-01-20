use super::*;

fn default_as_false() -> bool {
    false
}

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

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaString {
    pattern: Option<String>, //TODO Support regex-
    min_length: Option<u64>,
    max_length: Option<u64>,
    #[serde(rename = "enum")]
    enum_: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
pub enum OApiSchemaAdditionalItem {
    Any(bool),
    Obj(OApiSchemaObject),
}

impl std::default::Default for OApiSchemaAdditionalItem {
    fn default() -> Self {
        OApiSchemaAdditionalItem::Any(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaArray {
    additional_items: OApiSchemaAdditionalItem,
    max_items: Option<u64>,
    min_items: Option<u64>,
    #[serde(default = "default_as_false")]
    unique_items: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaObject {
    required: Vec<String>,
    properties: HashMap<String, OApiSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaNumeric {
    multiple_of: Option<u64>,
    #[serde(flatten)]
    maximum: Option<OApiNumericMaximum>,
    #[serde(flatten)]
    minimum: Option<OApiNumericMinimum>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
pub enum OApiSchema {
    Obj(OApiSchemaObject),
    Array(OApiSchemaArray),
    Numeric(OApiSchemaNumeric),
    String(OApiSchemaString),
}
