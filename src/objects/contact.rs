use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiContact {
    name: Option<String>,
    url: Option<Url>,
    email: Option<String>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
