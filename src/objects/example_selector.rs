use super::*;

/// ## A selector between a single example or multiple examples
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable, OApiCheck)]
#[serde(rename_all = "camelCase")]
pub enum OApiExampleSelector {
    Single(OApiExample),
    Multiple(Vec<OApiExample>),
}
