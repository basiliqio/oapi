use thiserror::Error;

/// # An error throwable by [OApi](crate)
#[derive(Error, Debug)]
pub enum OApiError {
    #[error("The OpenApi document check has failed at `{0}`: {1}")]
    OApiCheck(String, String),
    #[error("The `{0}` extension key doesn't exists")]
    NoExtKey(String),
    #[error(transparent)]
    SppparseError(#[from] sppparse::SparseError),
}
