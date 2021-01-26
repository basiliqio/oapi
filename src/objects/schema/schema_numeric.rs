use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaNumeric<NumericExt, DiscriminatorExt, ExternalDocExt> {
    multiple_of: Option<Option<u64>>,
    #[serde(flatten)]
    maximum: Option<OApiNumericMaximum>,
    #[serde(flatten)]
    minimum: Option<OApiNumericMinimum>,
    format: Option<OApiNumericFormat>,
    nullable: Option<OperatorSelector<bool>>,
    read_only: Option<OperatorSelector<bool>>,
    write_only: Option<OperatorSelector<bool>>,
    example: Option<OperatorSelector<Value>>,
    deprecated: Option<OperatorSelector<bool>>,
    discriminator: Option<OperatorSelector<OApiDiscriminator<DiscriminatorExt>>>,
    external_docs: Option<OperatorSelector<OApiExternalDocumentation<ExternalDocExt>>>,
    #[serde(flatten)]
    extension: NumericExt,
}
