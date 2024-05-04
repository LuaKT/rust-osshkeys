use backtrace::Backtrace;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// The [Result](https://doc.rust-lang.org/std/result/enum.Result.html) alias of this crate
pub type OsshResult<T> = Result<T, Error>;

/// The error type of this crate
pub struct Error {
    kind: ErrorKind,
    inner: Option<Box<dyn StdError + Send + Sync + 'static>>,
    bt: Backtrace,
}

impl Error {
    #[inline]
    pub(crate) fn from_kind(kind: ErrorKind) -> Self {
        Error {
            kind,
            inner: None,
            bt: Backtrace::new(),
        }
    }

    #[inline]
    pub(crate) fn with_error<E: StdError + Send + Sync + 'static>(kind: ErrorKind, err: E) -> Self {
        Error {
            kind,
            inner: Some(err.into()),
            bt: Backtrace::new(),
        }
    }

    /// Get the kind of the error
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn backtrace(&self) -> &Backtrace {
        &self.bt
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        writeln!(f, "OsshError {{")?;
        write!(f, "Kind: {:?} => \"{}\"", self.kind, self.kind)?;
        if let Some(cause) = &self.inner {
            write!(f, "\nCaused: {:?}", cause)?;
        }
        write!(f, "\nBackTrace: \n{:?}", self.bt)?;
        write!(f, "\n}}")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.kind)?;
        if let Some(cause) = &self.inner {
            write!(f, "; Caused by: {}", cause)?;
        }
        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        fn hack<'a>(e: &'a (dyn StdError + Send + Sync + 'static)) -> &'a (dyn StdError + 'static) {
            unsafe {
                // We need to remove Send and Sync trait bound,
                // but Rust currently not support trait upcasting. :(
                // So we cast it to pointer then cast it back.
                (e as *const (dyn StdError + Send + Sync)).as_ref().unwrap()
            }
        }

        self.inner.as_ref().map(|e| hack(e.as_ref()))
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::from_kind(kind)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::with_error(ErrorKind::IOError, err)
    }
}
impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Self::with_error(ErrorKind::FmtError, err)
    }
}
impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Self {
        Self::with_error(ErrorKind::OpenSslError, err)
    }
}
impl From<ed25519_dalek::SignatureError> for Error {
    fn from(err: ed25519_dalek::SignatureError) -> Self {
        Self::with_error(ErrorKind::Ed25519Error, err)
    }
}
impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Self::with_error(ErrorKind::Base64Error, err)
    }
}
impl From<bcrypt_pbkdf::Error> for Error {
    fn from(err: bcrypt_pbkdf::Error) -> Self {
        use bcrypt_pbkdf::Error::*;
        let kind = match err {
            InvalidParamLen => ErrorKind::InvalidLength,
            InvalidRounds => ErrorKind::InvalidArgument,
            _ => ErrorKind::Unknown,
        };
        Self::with_error(kind, err)
    }
}

#[cfg(feature = "rustcrypto-cipher")]
impl From<cipher::InvalidLength> for Error {
    fn from(err: cipher::InvalidLength) -> Self {
        Self::with_error(ErrorKind::InvalidKeyIvLength, err)
    }
}
#[cfg(feature = "rustcrypto-cipher")]
impl From<cipher::inout::PadError> for Error {
    fn from(err: cipher::inout::PadError) -> Self {
        Self::with_error(ErrorKind::InvalidLength, err)
    }
}
#[cfg(feature = "rustcrypto-cipher")]
impl From<cipher::StreamCipherError> for Error {
    fn from(err: cipher::StreamCipherError) -> Self {
        Self::with_error(ErrorKind::InvalidLength, err)
    }
}
#[cfg(feature = "rustcrypto-cipher")]
impl From<cipher::block_padding::UnpadError> for Error {
    fn from(err: cipher::block_padding::UnpadError) -> Self {
        Self::with_error(ErrorKind::IncorrectPass, err)
    }
}

impl From<pem::PemError> for Error {
    fn from(_err: pem::PemError) -> Self {
        Self::from_kind(ErrorKind::InvalidPemFormat)
    }
}
impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Self::with_error(ErrorKind::InvalidLength, err)
    }
}

/// Indicate the reason of the error
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ErrorKind {
    /// The error is caused by OpenSSL, to get the underlying error, use [std::error::Error::source()](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)
    OpenSslError,
    /// The error is caused by ed25519-dalek, to get the underlying error, use [std::error::Error::source()](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)
    Ed25519Error,
    /// The error is caused by I/O error or reader error
    IOError,
    /// Can't format some data
    FmtError,
    /// The base64 string is invalid
    Base64Error,
    /// The argument passed into the function is invalid
    InvalidArgument,
    /// The key file has some invalid data in it
    InvalidKeyFormat,
    /// Currently not used...
    InvalidFormat,
    /// Some parts of the key are invalid
    InvalidKey,
    /// The key size is invalid
    InvalidKeySize,
    /// The slice length is invalid
    InvalidLength,
    /// The elliptic curve is not supported
    UnsupportCurve,
    /// The encrypt cipher is not supported
    UnsupportCipher,
    /// The passphrase is incorrect, can't decrypt the key
    IncorrectPass,
    /// The key type is not the desired one
    TypeNotMatch,
    /// The key type is not supported
    UnsupportType,
    /// The key file's PEM part is invalid
    InvalidPemFormat,
    /// The key or IV length can't meet the cipher's requirement
    InvalidKeyIvLength,
    /// Something shouldn't happen but it DID happen...
    Unknown,
}

impl ErrorKind {
    /// Get the description of the kind
    pub fn description(self) -> &'static str {
        use ErrorKind::*;

        match self {
            OpenSslError => "OpenSSL Error",
            Ed25519Error => "Ed25519 Error",
            IOError => "I/O Error",
            FmtError => "Formatter Error",
            Base64Error => "Base64 Error",
            InvalidArgument => "Invalid Argument",
            InvalidKeyFormat => "Invalid Key Format",
            InvalidFormat => "Invalid Format",
            InvalidKey => "Invalid Key",
            InvalidKeySize => "Invalid Key Size",
            InvalidLength => "Invalid Length",
            UnsupportCurve => "Unsupported Elliptic Curve",
            UnsupportCipher => "Unsupported Cipher",
            IncorrectPass => "Incorrect Passphrase",
            TypeNotMatch => "Key Type Not Match",
            UnsupportType => "Unsupported Key Type",
            InvalidPemFormat => "Invalid PEM Format",
            InvalidKeyIvLength => "Invalid Key/IV Length",
            Unknown => "Unknown Error",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}
