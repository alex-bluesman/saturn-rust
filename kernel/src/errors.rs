extern crate alloc;

use core::fmt;
use core::error::Error;
use alloc::boxed::Box;

#[derive(Debug)]
pub struct KernelError {
    kind: ErrorKind,
    source: Option<anyhow::Error>
}

impl KernelError {
    pub fn new<E>(error_kind: ErrorKind, error_reason: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        let err = anyhow::anyhow!(error_reason.into());
        KernelError {
            kind: error_kind,
            source: err.into(),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnknownError,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::UnknownError => write!(f, "unknown error"),
        }
    }
}
