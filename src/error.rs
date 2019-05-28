use std::fmt;
use std::error;

use platform_impl;

#[derive(Debug)]
pub enum ExternalError {
    NotSupported(NotSupportedError),
    Os(OsError),
}

#[derive(Clone)]
pub struct NotSupportedError {
    _marker: (),
}

#[derive(Debug)]
pub struct OsError {
    line: u32,
    file: &'static str,
    error: platform_impl::OsError,
}

impl NotSupportedError {
    #[inline]
    pub(crate) fn new() -> NotSupportedError {
        NotSupportedError {
            _marker: ()
        }
    }
}

impl OsError {
    pub(crate) fn new(line: u32, file: &'static str, error: platform_impl::OsError) -> OsError {
        OsError {
            line,
            file,
            error,
        }
    }
}

macro_rules! os_error {
    ($error:expr) => {{
        crate::error::OsError::new(line!(), file!(), $error)
    }}
}

impl fmt::Display for OsError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.pad(&format!("os error at {}:{}: {}", self.file, self.line, self.error))
    }
}

impl fmt::Display for ExternalError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ExternalError::NotSupported(e) => e.fmt(formatter),
            ExternalError::Os(e) => e.fmt(formatter),
        }
    }
}

impl fmt::Debug for NotSupportedError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.debug_struct("NotSupportedError").finish()
    }
}

impl fmt::Display for NotSupportedError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.pad("the requested operation is not supported by Winit")
    }
}

impl error::Error for OsError {}
impl error::Error for ExternalError {}
impl error::Error for NotSupportedError {}