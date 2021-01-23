use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable, OApiCheck)]
#[serde(rename_all = "camelCase")]
pub enum OApiExampleSelector {
    Single(OApiExample),
    Multiple(Vec<OApiExample>),
}
