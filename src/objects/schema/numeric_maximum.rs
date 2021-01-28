use super::*;

/// ## A selector between the inclusive or exclusive way to express upper numeric bound
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable, OApiCheck)]
pub enum OApiNumericMaximum {
    #[serde(rename = "maximum")]
    Inclusive(u64),
    #[serde(rename = "exclusiveMaximum")]
    Exclusive(u64),
}
