use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiStringFormat {
    Byte,
    Binary,
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    Password,
}
