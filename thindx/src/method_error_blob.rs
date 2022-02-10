use crate::*;
use crate::d3d::*;

use std::fmt::{self, Debug, Display, Formatter};



/// { kind: [ErrorKind], method, errors }
#[derive(Clone)]
pub struct MethodErrorBlob {
    pub(crate) error:   MethodError,
    pub(crate) errors:  TextBlob,
}

impl MethodErrorBlob {
    /// Returns the corresponding [ErrorKind] for this error.
    pub fn kind(&self) -> ErrorKind { self.error.kind() }

    pub(crate) fn method(&self) -> &'static str { self.error.method() }
}

impl From<MethodErrorBlob> for ErrorKind { fn from(error: MethodErrorBlob) -> ErrorKind { error.kind() } }
//impl From<ErrorKind> for MethodErrorBlob { fn from(error: ErrorKind   ) -> Self { Self { kind: error, method: None, errors: Default::default() } } }
impl From<MethodError> for MethodErrorBlob { fn from(error: MethodError) -> Self { Self { error, errors: Default::default() } } }

impl std::error::Error for MethodErrorBlob {}

impl Debug for MethodErrorBlob {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut ds = fmt.debug_struct("MethodErrorBlob");
        ds.field("kind", &self.kind());
        ds.field("method", &self.method());
        if !self.errors.is_empty() {
            ds.field("errors", &self.errors.to_utf8_lossy());
        }
        ds.finish()
    }
}

impl Display for MethodErrorBlob {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{} failed ({:?})", self.method(), self.kind())?;
        if !self.errors.is_empty() {
            write!(fmt, "\n{}\n", self.errors.to_utf8_lossy())?;
        }
        Ok(())
    }
}
