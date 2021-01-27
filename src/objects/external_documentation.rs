use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExternalDocumentation {
    url: Url,
    #[serde(default)]
    description: Option<String>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
