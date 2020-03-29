//! Errors for Games-rs

use std::fmt;

/// Base Error code for Coin Toss (5XXX)
pub const BASE_COIN_TOSS_ERROR_CODE: i32 = 5000;

/// A Generic Error for simplification
#[derive(Debug, failure::Fail, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GenericError {
    /// Error Code
    pub error_code: i32,
    /// Error message
    pub error_message: String,
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error ({}): {}", self.error_code, self.error_message)
    }
}

/// Error code trait
/// Note: This trait is Temporary, waiting for std::proces::Termination to be stabalized. (#43301)
pub trait ErrorCode: failure::Fail {
    /// Return error code
    fn error_code(&self) -> i32;
}

impl<E: ErrorCode> From<E> for GenericError {
    fn from(err: E) -> Self {
        Self {
            error_code: err.error_code(),
            error_message: format!("{:?}", err),
        }
    }
}
