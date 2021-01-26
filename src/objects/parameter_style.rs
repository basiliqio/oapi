use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable, OApiCheck)]
#[serde(rename_all = "camelCase")]
pub enum OApiParameterStyle {
    Form,
    Simple,
}
