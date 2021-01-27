use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiRequestBody {
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    content: HashMap<String, OApiMediaType>, // TODO Check that key is a valid media type
    #[serde(default)]
    required: bool,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
