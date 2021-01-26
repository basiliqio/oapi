use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaString<StringExt, DiscriminatorExt, ExternalDocExt> {
    pattern: Option<String>, //TODO Support regex-
    min_length: Option<u64>,
    max_length: Option<u64>,
    format: Option<OApiStringFormat>,
    #[serde(rename = "enum")]
    enum_: Option<Vec<String>>,
    nullable: Option<OperatorSelector<bool>>,
    read_only: Option<OperatorSelector<bool>>,
    write_only: Option<OperatorSelector<bool>>,
    example: Option<OperatorSelector<Value>>,
    deprecated: Option<OperatorSelector<bool>>,
    discriminator: Option<OperatorSelector<OApiDiscriminator<DiscriminatorExt>>>,
    external_docs: Option<OperatorSelector<OApiExternalDocumentation<ExternalDocExt>>>,
    #[serde(flatten)]
    extension: StringExt,
}
