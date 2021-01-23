use thiserror::Error;

/// # An error throwable by [OApi](crate)
#[derive(Error, Debug)]
pub enum OApiError {
    #[error(transparent)]
    SppparseError(#[from] sppparse::SparseError),
}
