use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, OApiCheck)]
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
