use super::*;
use std::convert::From;
use std::ops::Deref;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct OApi {
    doc: SparseRoot<OApiDocument>,
}

impl OApi {
    pub fn check(&self) -> Result<(), OApiError> {
        self.doc().root_get()?.oapi_check(self.doc(), &mut vec![])
    }

    pub fn new(doc: SparseRoot<OApiDocument>) -> Self {
        OApi { doc }
    }
}

impl From<SparseRoot<OApiDocument>> for OApi {
    fn from(doc: SparseRoot<OApiDocument>) -> Self {
        OApi::new(doc)
    }
}

impl Deref for OApi {
    type Target = SparseRoot<OApiDocument>;

    fn deref(&self) -> &Self::Target {
        &self.doc
    }
}
