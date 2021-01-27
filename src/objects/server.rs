use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServer {
    url: String,
    description: Option<String>,
    #[serde(default)]
    variables: HashMap<String, OApiServerVariable>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
