use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
pub enum OApiSchemaAdditionalItem<
    ObjectExt,
    ArrayExt,
    NumericExt,
    StringExt,
    DiscriminatorExt,
    ExternalDocExt,
> {
    Any(bool),
    Obj(
        Box<
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
    ),
}

impl<ObjectExt, ArrayExt, NumericExt, StringExt, DiscriminatorExt, ExternalDocExt>
    std::default::Default
    for OApiSchemaAdditionalItem<
        ObjectExt,
        ArrayExt,
        NumericExt,
        StringExt,
        DiscriminatorExt,
        ExternalDocExt,
    >
{
    fn default() -> Self {
        OApiSchemaAdditionalItem::Any(false)
    }
}
