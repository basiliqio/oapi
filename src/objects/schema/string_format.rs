use super::*;

/// ## The string format authorized by the OAPI specs
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable, OApiCheck)]
#[serde(rename_all = "camelCase")]
pub enum OApiStringFormat {
    Byte,
    Binary,
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    Password,
}
