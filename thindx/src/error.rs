use crate::*;
use crate::d3d::*;

use winapi::shared::winerror::*;
use winapi::um::d3dcommon::ID3DBlob;

use std::fmt::{self, Debug, Display, Formatter};



/// { kind: [ErrorKind], method, errors }
#[derive(Clone)]
pub struct Error {
    pub(crate) kind:       ErrorKind,
    pub(crate) method:     Option<&'static str>,
    pub(crate) errors:     CompilerDiagnostics,
}

impl Error {
    pub fn kind(&self) -> ErrorKind { self.kind }

    pub(crate) fn new(method: &'static str, kind: ErrorKind) -> Self { Self { kind, method: Some(method), errors: Default::default() } }

    pub(crate) fn check(method: &'static str, hr: HRESULT) -> Result<(), Self> {
        if !SUCCEEDED(hr) {
            Err(Self {
                kind:   ErrorKind(hr),
                method: Some(method),
                errors: CompilerDiagnostics::default(),
            })
        } else {
            Ok(())
        }
    }

    /// ### Safety
    ///
    /// * If `!SUCCEEDED(hr)`, this accesses and takes over ownership of `errors` and returns `Err(...)`.
    /// * Otherwise, `errors` is left untouched.
    pub(crate) unsafe fn check_blob(method: &'static str, hr: HRESULT, errors: *mut ID3DBlob) -> Result<(), Self> {
        if !SUCCEEDED(hr) {
            let errors = CompilerDiagnostics::new(ReadOnlyBlob::from_raw_opt(errors));
            Err(Self {
                kind:   ErrorKind(hr),
                method: Some(method),
                errors,
            })
        } else {
            Ok(())
        }
    }
}

impl From<Error> for ErrorKind { fn from(error: Error       ) -> ErrorKind { error.kind } }
//impl From<ErrorKind> for Error { fn from(error: ErrorKind   ) -> Error { Error { kind: error, method: None, errors: None } } }

impl std::error::Error for Error {}

impl Debug for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut ds = fmt.debug_struct("Error");
        ds.field("kind", &self.kind);
        if let Some(method) = self.method.as_ref() {
            ds.field("method", method);
        }
        if !self.errors.is_empty() {
            ds.field("errors", &self.errors.to_utf8_lossy());
        }
        ds.finish()
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let method = self.method.unwrap_or("thindx method");
        write!(fmt, "{} failed ({:?})", method, self.kind)?;
        if !self.errors.is_empty() {
            write!(fmt, "\n{}\n", self.errors.to_utf8_lossy())?;
        }
        Ok(())
    }
}
