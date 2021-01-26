use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaArray<
    ArrayExt,
    ObjectExt,
    NumericExt,
    StringExt,
    DiscriminatorExt,
    ExternalDocExt,
> {
    additional_items: Option<
        OApiSchemaAdditionalItem<
            ObjectExt,
            ArrayExt,
            NumericExt,
            StringExt,
            DiscriminatorExt,
            ExternalDocExt,
        >,
    >,
    max_items: Option<u64>,
    min_items: Option<u64>,
    items: Option<
        OperatorSelector<
            OApiSchema<
                ObjectExt,
                ArrayExt,
                NumericExt,
                StringExt,
                DiscriminatorExt,
                ExternalDocExt,
            >,
        >,
    >,
    unique_items: bool,
    // #[serde(flatten)]
    // common: OApiSchemaCommon<DiscriminatorExt, ExternalDocExt>,
    nullable: Option<OperatorSelector<bool>>,
    read_only: Option<OperatorSelector<bool>>,
    write_only: Option<OperatorSelector<bool>>,
    example: Option<OperatorSelector<Value>>,
    deprecated: Option<OperatorSelector<bool>>,
    discriminator: Option<OperatorSelector<OApiDiscriminator<DiscriminatorExt>>>,
    external_docs: Option<OperatorSelector<OApiExternalDocumentation<ExternalDocExt>>>,
    #[serde(flatten)]
    extension: ArrayExt,
}
