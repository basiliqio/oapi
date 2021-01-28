use super::*;

/// ## An OpenApi schema
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable, OApiCheck)]
#[serde(tag = "type")]
pub enum OApiSchema {
    #[serde(rename = "object")]
    Obj(Box<OperatorSelector<OApiSchemaObject>>),
    #[serde(rename = "array")]
    Array(Box<OperatorSelector<OApiSchemaArray>>),
    #[serde(rename = "number")]
    #[serde(alias = "integer")]
    Numeric(Box<OperatorSelector<OApiSchemaNumeric>>),
    #[serde(rename = "string")]
    String(Box<OperatorSelector<OApiSchemaString>>),
    #[serde(rename = "boolean")]
    Bool,
    #[serde(rename = "null")]
    Null,
}
