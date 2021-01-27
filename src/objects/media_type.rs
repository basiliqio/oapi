use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Default, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiMediaType {
    #[serde(default)]
    schema: Option<OperatorSelector<OApiSchema>>,
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
    #[serde(default)]
    encoding: HashMap<String, OApiEncoding>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
