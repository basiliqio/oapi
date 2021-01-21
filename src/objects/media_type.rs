use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiMediaType {
    schema: Option<OperatorSelector<OApiSchema>>,
    #[serde(flatten)]
    example: Option<OApiExampleSelector>,
    #[serde(default)]
    encoding: HashMap<String, OApiEncoding>,
}
