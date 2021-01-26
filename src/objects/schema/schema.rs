use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
#[serde(tag = "type")]
pub enum OApiSchema<ObjectExt, ArrayExt, NumericExt, StringExt, DiscriminatorExt, ExternalDocExt> {
    #[serde(rename = "object")]
    Obj(
        Box<
            OperatorSelector<
                OApiSchemaObject<
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
    #[serde(rename = "array")]
    Array(
        Box<
            OperatorSelector<
                OApiSchemaArray<
                    ArrayExt,
                    ObjectExt,
                    NumericExt,
                    StringExt,
                    DiscriminatorExt,
                    ExternalDocExt,
                >,
            >,
        >,
    ),
    #[serde(rename = "number")]
    #[serde(alias = "integer")]
    Numeric(Box<OperatorSelector<OApiSchemaNumeric<NumericExt, DiscriminatorExt, ExternalDocExt>>>),
    #[serde(rename = "string")]
    String(Box<OperatorSelector<OApiSchemaString<StringExt, DiscriminatorExt, ExternalDocExt>>>),
    #[serde(rename = "boolean")]
    Bool,
    #[serde(rename = "null")]
    Null,
}
