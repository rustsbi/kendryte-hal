//! Custom error type for the xtask crate
//!
//! This enum represents various error types that can occur during the execution of xtask operations.

use thiserror::Error;

pub type XtaskResult<T> = Result<T, XtaskError>;

/// Custom error type for encryption and firmware generation operations.
#[derive(Error, Debug)]
pub enum XtaskError {
    /// Error for invalid encryption type specification.
    #[error("Invalid encryption type!")]
    InvalidEncryptionType,

    /// Wrapper for standard I/O errors.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Errors from AES encryption/decryption operations.
    #[error("Aes error: {0}")]
    AesError(String),

    /// Errors from RSA cryptographic operations.
    #[error("RSA error: {0}")]
    RsaError(#[from] rsa::errors::Error),

    /// Errors from SM2 signature operations.
    #[error("Sm2 error: {0}")]
    Sm2Error(#[from] Sm2Error),

    /// Errors when parsing RSA key components.
    #[error("RSA parse error: {0}")]
    RsaParseError(String),

    /// Errors when parsing ELF/OBJ files.
    #[error("ELF parsing error: {0}")]
    ElfParseError(String),

    /// Errors when processing ELF sections larger than supported size.
    #[error("Section size {0} is too large to fit in memory")]
    SectionSizeOverflow(u64),
}

#[derive(Error, Debug)]
pub enum Sm2Error {
    #[error("{0}")]
    EllipticCurveError(#[from] elliptic_curve::Error),
    #[error("{0}")]
    SignatureError(#[from] signature::Error),
}

impl From<elliptic_curve::Error> for XtaskError {
    fn from(err: elliptic_curve::Error) -> Self {
        XtaskError::Sm2Error(Sm2Error::EllipticCurveError(err))
    }
}

impl From<signature::Error> for XtaskError {
    fn from(err: signature::Error) -> Self {
        XtaskError::Sm2Error(Sm2Error::SignatureError(err))
    }
}
