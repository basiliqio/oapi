use super::*;

/// ## The OpenApi [server variables](https://swagger.io/specification/#server-variable-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[oapi(handler = "self._oapi_check")]
pub struct OApiServerVariable {
    /// The possible values for the variable
    #[serde(default)]
    #[serde(rename = "enum")]
    enum_: Vec<String>,
    /// The default value for this variable
    default: String,
    /// A description of this variable
    description: Option<String>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}

impl OApiServerVariable {
    fn _oapi_check(
        &self,
        _root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        if self.enum_.is_empty() {
            bread_crumb.push("enum".to_string());
            return Err(OApiError::OApiCheck(
                crate::check::connect_bread_crumbs(bread_crumb),
                "Enum shoud not be empty".to_string(),
            ));
        }
        if !self.enum_.contains(&self.default) {
            bread_crumb.push("default".to_string());
            return Err(OApiError::OApiCheck(
                crate::check::connect_bread_crumbs(bread_crumb),
                "default value should be present in `enum`".to_string(),
            ));
        }
        Ok(())
    }
}
