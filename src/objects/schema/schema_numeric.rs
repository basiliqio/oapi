use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaNumeric {
    multiple_of: Option<OperatorSelector<u64>>,
    #[serde(flatten)]
    maximum: Option<OperatorSelector<OApiNumericMaximum>>,
    #[serde(flatten)]
    minimum: Option<OperatorSelector<OApiNumericMinimum>>,
    format: Option<OApiNumericFormat>,
    #[serde(flatten)]
    common: OApiSchemaCommon,
}
