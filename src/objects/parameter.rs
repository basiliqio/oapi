use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiParameter {
    name: String,
    #[serde(rename = "in")]
    in_: OApiParameterLocation,
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
