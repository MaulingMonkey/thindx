use crate::*;
use crate::d3d::*;

use std::fmt::{self, Debug, Display, Formatter};



/// { error: [Error], errors: [TextBlob] }
#[derive(Clone)]
pub struct ErrorWithBlob {
    pub(crate) error:   Error,
    pub(crate) errors:  TextBlob,
}

impl ErrorWithBlob {
    /// Returns the corresponding [ErrorKind] for this error.
    pub fn kind(&self) -> ErrorKind { self.error.kind() }

    pub(crate) fn method(&self) -> &'static str { self.error.method() }
}

impl From<ErrorWithBlob> for ErrorKind { fn from(error: ErrorWithBlob) -> ErrorKind { error.kind() } }
//impl From<ErrorKind> for ErrorWithBlob { fn from(error: ErrorKind   ) -> Self { Self { error: ..., errors: Default::default() } } }
impl From<Error> for ErrorWithBlob { fn from(error: Error) -> Self { Self { error, errors: Default::default() } } }

impl std::error::Error for ErrorWithBlob {}

impl Debug for ErrorWithBlob {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut ds = fmt.debug_struct("ErrorWithBlob");
        ds.field("error", &self.error);
        if !self.errors.is_empty() {
            ds.field("errors", &self.errors.to_utf8_lossy());
        }
        ds.finish()
    }
}

impl Display for ErrorWithBlob {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{} failed ({:?})", self.method(), self.kind())?;
        if !self.errors.is_empty() {
            write!(fmt, "\n{}\n", self.errors.to_utf8_lossy())?;
        }
        Ok(())
    }
}
