use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(rename_all = "camelCase")]
pub enum OApiExampleSelector<ExampleExt> {
    Single(OApiExample<ExampleExt>),
    Multiple(Vec<OApiExample<ExampleExt>>),
}
