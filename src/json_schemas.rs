use super::*;

fn default_as_false() -> OperatorSelector<bool> {
    OperatorSelector::new_from_val(false)
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
pub enum OApiNumericMaximum {
    #[serde(rename = "maximum")]
    Inclusive(OperatorSelector<u64>),
    #[serde(rename = "exclusiveMaximum")]
    Exclusive(OperatorSelector<u64>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable)]
pub enum OApiNumericMinimum {
    #[serde(rename = "minimum")]
    Inclusive(OperatorSelector<u64>),
    #[serde(rename = "exclusiveMinimum")]
    Exclusive(OperatorSelector<u64>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaString {
    pattern: Option<OperatorSelector<String>>, //TODO Support regex-
    min_length: Option<OperatorSelector<u64>>,
    max_length: Option<OperatorSelector<u64>>,
    #[serde(rename = "enum")]
    enum_: Option<OperatorSelector<Vec<OperatorSelector<String>>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
pub enum OApiSchemaAdditionalItem {
    Any(OperatorSelector<bool>),
    Obj(Box<OperatorSelector<OApiSchemaObject>>),
}

impl std::default::Default for OApiSchemaAdditionalItem {
    fn default() -> Self {
        OApiSchemaAdditionalItem::Any(OperatorSelector::new_from_val(false))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaArray {
    additional_items: OperatorSelector<OApiSchemaAdditionalItem>,
    max_items: Option<OperatorSelector<u64>>,
    min_items: Option<OperatorSelector<u64>>,
    #[serde(default = "default_as_false")]
    unique_items: OperatorSelector<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaObject {
    required: OperatorSelector<Vec<OperatorSelector<String>>>,
    properties: OperatorSelector<HashMap<String, OperatorSelector<OApiSchema>>>,
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
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
pub enum OApiSchema {
    Obj(Box<OperatorSelector<OApiSchemaObject>>),
    Array(Box<OperatorSelector<OApiSchemaArray>>),
    Numeric(OperatorSelector<OApiSchemaNumeric>),
    String(OperatorSelector<OApiSchemaString>),
}
