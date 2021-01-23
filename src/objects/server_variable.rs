use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServerVariable {
    #[serde(default)]
    #[serde(rename = "enum")]
    enum_: Vec<String>,
    default: String,
    description: Option<String>,
}

impl OApiCheckTrait for OApiServerVariable {
    fn oapi_check(
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
