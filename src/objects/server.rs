use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServer<ServerExt, ServerVarExt> {
    url: String,
    description: Option<String>,
    #[serde(default)]
    variables: HashMap<String, OApiServerVariable<ServerVarExt>>,
    #[serde(flatten)]
    extension: ServerExt,
}
