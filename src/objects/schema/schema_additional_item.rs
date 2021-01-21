use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable)]
pub enum OApiSchemaAdditionalItem {
    Any(bool),
    Obj(Box<OperatorSelector<OApiSchema>>),
}

impl std::default::Default for OApiSchemaAdditionalItem {
    fn default() -> Self {
        OApiSchemaAdditionalItem::Any(false)
    }
}
