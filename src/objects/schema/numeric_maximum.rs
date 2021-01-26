use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable, OApiCheck)]
pub enum OApiNumericMaximum {
    #[serde(rename = "maximum")]
    Inclusive(u64),
    #[serde(rename = "exclusiveMaximum")]
    Exclusive(u64),
}
