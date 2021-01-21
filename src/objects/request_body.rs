use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiRequestBody {
    description: Option<String>,
    content: HashMap<String, OApiMediaType>,
    required: bool,
}
