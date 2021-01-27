use super::*;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiHeader {
    description: Option<String>,
    #[serde(default)]
    required: bool,
    #[serde(default)]
    deprecated: bool,
    #[serde(default)]
    allow_empty_value: bool,
    style: Option<OApiParameterStyle>,
    #[serde(default)]
    explode: bool,
    #[serde(default)]
    allow_reserved: bool,
    schema: Option<OperatorSelector<OApiSchema>>,
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
