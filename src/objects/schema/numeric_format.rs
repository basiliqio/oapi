use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable, OApiCheck)]
#[serde(rename_all = "camelCase")]
pub enum OApiNumericFormat {
    Int32,
    Int64,
    Float,
    Double,
}
