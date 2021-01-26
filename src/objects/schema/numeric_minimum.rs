use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable, OApiCheck)]
pub enum OApiNumericMinimum {
    #[serde(rename = "minimum")]
    Inclusive(u64),
    #[serde(rename = "exclusiveMinimum")]
    Exclusive(u64),
}
