use super::*;

/// ## A selector for additional items
///
/// Could be a boolean telling if additional items are allowed or not,
/// or could be a schema describing how they should be structured.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Sparsable, OApiCheck)]
pub enum OApiSchemaAdditionalItem {
    Any(bool),
    Obj(Box<OperatorSelector<OApiSchema>>),
}

impl std::default::Default for OApiSchemaAdditionalItem {
    fn default() -> Self {
        OApiSchemaAdditionalItem::Any(false)
    }
}
