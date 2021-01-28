use super::*;

/// ## The OpenApi [link object](https://swagger.io/specification/#link-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[oapi(handler = "self._oapi_check")]
pub struct OApiLink {
    /// A description of the link
    #[serde(default)]
    description: Option<String>,
    /// A reference to an operation, may not be defined if `operation_id` is defined
    #[serde(default)]
    operation_ref: Option<SparseRefRawInline<OApiOperation>>,
    /// The id of an operation, may not be defined if `operation_ref` is defined
    #[serde(default)]
    operation_id: Option<String>,
    /// Parameters to use for this link
    #[serde(default)]
    parameters: HashMap<String, String>,
    /// A request body expression to use
    request_body: Option<String>,
    /// A list of server to use as target
    #[serde(default)]
    server: Option<OApiServer>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
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
