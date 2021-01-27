use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[oapi(handler = "self._oapi_check")]
pub struct OApiLink {
    operation_ref: Option<String>, //TODO Handle inline links
    operation_id: Option<String>,
    #[serde(default)]
    parameters: HashMap<String, String>,
    request_body: Option<String>,
    description: Option<String>,
    #[serde(default)]
    server: Option<OApiServer>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}

impl OApiLink {
    fn check_target(&self, bread_crumb: &mut Vec<String>) -> Result<(), OApiError> {
        match (self.operation_id(), self.operation_ref()) {
            (Some(_), Some(_)) | (None, None) => {
                return Err(OApiError::OApiCheck(
                    crate::check::connect_bread_crumbs(bread_crumb),
                    "Either `operationRef` or `operationId` should be defined".to_string(),
                ));
            }
            // (None, Some(x)) => match x.get() {
            //     Ok(_) => (),
            //     Err(sppparse::SparseError::BadPointer) => {
            //         bread_crumb.push("operation_ref".to_string());
            //         return Err(OApiError::OApiCheck(
            //             crate::check::connect_bread_crumbs(bread_crumb),
            //             "The `operationRef` pointer is invalid".to_string(),
            //         ));
            //     }
            //     Err(err) => return Err(OApiError::SppparseError(err)),
            // },
            _ => (),
        }

        Ok(())
    }

    fn _oapi_check(
        &self,
        _root: &Rc<RefCell<SparseState>>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.check_target(bread_crumb)?;
        // if let Some(opid) = self.operation_id() {
        // 	if root.root_get()?.get_operation_id(&opid).is_none() {
        // 		bread_crumb.push("operation_id".to_string());
        // 		return Err(OApiError::OApiCheck(
        // 			crate::check::connect_bread_crumbs(bread_crumb),
        // 			"The `operationId` doesn't exists".to_string(),
        // 		));
        // 	}
        // }
        Ok(())
    }
}
