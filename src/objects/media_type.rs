use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Default, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiMediaType<
    MediaTypeExt,
    ExampleExt,
    EncodingExt,
    HeaderExt,
    ObjectExt,
    ArrayExt,
    NumericExt,
    StringExt,
    DiscriminatorExt,
    ExternalDocExt,
> {
    schema: Option<
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
    #[serde(flatten)]
    example: Option<OApiExampleSelector<ExampleExt>>,
    #[serde(default)]
    encoding: HashMap<
        String,
        OApiEncoding<
            EncodingExt,
            HeaderExt,
            ExampleExt,
            ObjectExt,
            ArrayExt,
            NumericExt,
            StringExt,
            DiscriminatorExt,
            ExternalDocExt,
        >,
    >,
    #[serde(flatten)]
    extension: MediaTypeExt,
}
