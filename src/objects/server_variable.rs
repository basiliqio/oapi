use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiServerVariable<ServerVariableExt> {
    #[serde(default)]
    #[serde(rename = "enum")]
    enum_: Vec<String>,
    default: String,
    description: Option<String>,
    #[serde(flatten)]
    extension: ServerVariableExt,
}

// impl<ServerVariableExt> OApiCheckTrait for OApiServerVariable<ServerVariableExt>
// where
//     ServerVariableExt: OApiExtensionRequirements,
// {
//     fn oapi_check_inner(
//         &self,
//         root: &SparseRoot<OApiDocument>,
//         bread_crumb: &mut Vec<String>,
//     ) -> Result<(), OApiError> {
//         self.enum_.oapi_check(root, bread_crumb)?;
//         self.default.oapi_check(root, bread_crumb)?;
//         self.description.oapi_check(root, bread_crumb)?;
//         Ok(())
//     }

//     fn oapi_check(
//         &self,
//         root: &SparseRoot<OApiDocument>,
//         bread_crumb: &mut Vec<String>,
//     ) -> Result<(), OApiError> {
//         if self.enum_.is_empty() {
//             bread_crumb.push("enum".to_string());
//             return Err(OApiError::OApiCheck(
//                 crate::check::connect_bread_crumbs(bread_crumb),
//                 "Enum shoud not be empty".to_string(),
//             ));
//         }
//         if !self.enum_.contains(&self.default) {
//             bread_crumb.push("default".to_string());
//             return Err(OApiError::OApiCheck(
//                 crate::check::connect_bread_crumbs(bread_crumb),
//                 "default value should be present in `enum`".to_string(),
//             ));
//         }
//         self.oapi_check_inner(root, bread_crumb)
//     }
// }
