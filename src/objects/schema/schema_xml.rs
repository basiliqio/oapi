use super::*;

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
