use super::*;

/// ## Location possible for parameters
#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Clone, Sparsable, OApiCheck)]
#[serde(rename_all = "camelCase")]
pub enum OApiParameterLocation {
    Query,
    Header,
    Path,
    Cookie,
}
