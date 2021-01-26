use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSchemaObject<
    ObjectExt,
    ArrayExt,
    NumericExt,
    StringExt,
    DiscriminatorExt,
    ExternalDocExt,
> {
    required: Vec<String>,
    properties: Option<
        OperatorSelector<
            HashMap<
                String,
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
        >,
    >,
    max_properties: Option<OperatorSelector<u64>>,
    min_properties: Option<OperatorSelector<u64>>,
    nullable: Option<OperatorSelector<bool>>,
    read_only: Option<OperatorSelector<bool>>,
    write_only: Option<OperatorSelector<bool>>,
    example: Option<OperatorSelector<Value>>,
    deprecated: Option<OperatorSelector<bool>>,
    discriminator: Option<OperatorSelector<OApiDiscriminator<DiscriminatorExt>>>,
    external_docs: Option<OperatorSelector<OApiExternalDocumentation<ExternalDocExt>>>,
    #[serde(flatten)]
    extension: ObjectExt,
}
