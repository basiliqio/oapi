use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[oapi(handler = "self._oapi_check")]
pub struct OApiLink {
    #[serde(default)]
    operation_ref: Option<SparseRefRawInline<OApiOperation>>,
    #[serde(default)]
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
    fn _oapi_check(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        match (self.operation_id(), self.operation_ref()) {
            (None, None) | (Some(_), Some(_)) => Err(OApiError::OApiCheck(
                crate::check::connect_bread_crumbs(bread_crumb),
                "Either `operationId` or `operationRef` should be defined".to_string(),
            )),
            (Some(opid), None) => root
                .root_get()?
                .get_operation_id(opid)
                .map(|_x| ())
                .ok_or_else(|| {
                    OApiError::OApiCheck(
                        crate::check::connect_bread_crumbs(bread_crumb),
                        "`operationId` should be defined in the document".to_string(),
                    )
                }),
            _ => Ok(()),
        }
    }
}
