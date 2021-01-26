use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiSchemaCommon<DiscriminatorExt, ExternalDocExt> {
    nullable: Option<OperatorSelector<bool>>,
    read_only: Option<OperatorSelector<bool>>,
    write_only: Option<OperatorSelector<bool>>,
    example: Option<OperatorSelector<Value>>,
    deprecated: Option<OperatorSelector<bool>>,
    discriminator: Option<OperatorSelector<OApiDiscriminator<DiscriminatorExt>>>,
    external_docs: Option<OperatorSelector<OApiExternalDocumentation<ExternalDocExt>>>,
}
