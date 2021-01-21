use super::*;

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
