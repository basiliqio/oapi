use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServer {
    url: String,
    description: Option<String>,
    #[serde(default)]
    variables: HashMap<String, OApiServerVariable>,
}
