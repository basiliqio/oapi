use super::*;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

/// ## The string format authorized by the OAPI specs
#[derive(
    Debug, Deserialize_enum_str, Serialize_enum_str, Clone, PartialEq, Sparsable, OApiCheck,
)]
#[serde(rename_all = "camelCase")]
pub enum OApiStringFormat {
    Byte,
    Binary,
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    Password,
    /// Formats not defined by the OpenAPI specs
    #[serde(other)]
    Other(String),
}
