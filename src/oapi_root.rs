use super::*;
use std::convert::From;
use std::ops::Deref;

/// ## OpenApi document
///
/// This is the main struct of this crate. It contains a fully parsed OpenApi Document
/// wrapped in a [SparseRoot](sppparse::SparseRoot) to allow for Sparse Pointer
/// expansion.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct OApi {
    doc: SparseRoot<OApiDocument>,
}

impl OApi {
    /// Check the document for logic errors
    pub fn check(&self) -> Result<(), OApiError> {
        self.doc().root_get()?.oapi_check(self.doc(), &mut vec![])
    }

    /// Create a new [OApi](crate::OApi)
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
