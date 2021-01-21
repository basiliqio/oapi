use super::*;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiNumericFormat {
	Int32,
	Int64,
	Float,
	Double
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiStringFormat {
	Byte,
	Binary,
	Date,
	#[serde(rename = "date-time")]
	DateTime,
	Password
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaString {
    pattern: Option<OperatorSelector<String>>, //TODO Support regex-
    min_length: Option<OperatorSelector<u64>>,
	max_length: Option<OperatorSelector<u64>>,
	format: Option<OApiStringFormat>,
    #[serde(rename = "enum")]
	enum_: Option<OperatorSelector<Vec<OperatorSelector<String>>>>,
	#[serde(flatten)]
	common: OApiSchemaCommon,
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

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaArray {
    additional_items: OperatorSelector<OApiSchemaAdditionalItem>,
    max_items: Option<OperatorSelector<u64>>,
	min_items: Option<OperatorSelector<u64>>,
	items: Option<OperatorSelector<OApiSchema>>,
    #[serde(default)]
	unique_items: OperatorSelector<bool>,
	#[serde(flatten)]
	common: OApiSchemaCommon,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaObject {
    required: OperatorSelector<Vec<OperatorSelector<String>>>,
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
	property_name: OperatorSelector<String>,
	mapping: OperatorSelector<HashMap<String, String>>
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaXml {
	name: Option<OperatorSelector<String>>,
	namespace: Option<OperatorSelector<String>>,
	prefix: Option<OperatorSelector<String>>,
	attribute: Option<OperatorSelector<bool>>,
	wrapped: Option<OperatorSelector<bool>>,
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
    Null
}
