use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiHeader {
    description: Option<String>,
    required: bool,
    deprecated: bool,
    allow_empty_value: bool,
    style: Option<OApiParameterStyle>,
    explode: Option<bool>,
    allow_reserved: Option<bool>,
    schema: Option<OperatorSelector<OApiSchema>>,
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
}
