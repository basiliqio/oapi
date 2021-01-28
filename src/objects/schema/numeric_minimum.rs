use super::*;

/// ## A selector between the inclusive or exclusive way to express lower numeric bound
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable, OApiCheck)]
pub enum OApiNumericMinimum {
    #[serde(rename = "minimum")]
    Inclusive(u64),
    #[serde(rename = "exclusiveMinimum")]
    Exclusive(u64),
}
