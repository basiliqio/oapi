use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiResponse {
    description: String,
    #[serde(default)]
    headers: HashMap<String, OApiHeader>,
    #[serde(default)]
    content: HashMap<String, OApiMediaType>,
    #[serde(default)]
    links: HashMap<String, OApiLink>,
}
