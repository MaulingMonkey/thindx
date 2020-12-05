use crate::d3d::{ReadOnlyBlob, OptTextBlob};

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::Utf8Error;



#[derive(Clone, Default)]
pub struct CompilerDiagnostics(OptTextBlob);

impl CompilerDiagnostics {
    pub fn new(value: impl Into<Self>) -> Self { value.into() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn to_utf8_lossy(&self) -> Cow<str> { self.0.to_utf8_lossy() }
    pub fn to_utf8(&self) -> Result<&str, Utf8Error> { self.0.to_utf8() }
    // TODO: parsing diagnostics iterator?
}

impl Debug   for CompilerDiagnostics { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "CompilerDiagnostics({:?})", self.to_utf8_lossy()) } }
impl Display for CompilerDiagnostics { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.to_utf8_lossy(), fmt) } }

impl From<Option<ReadOnlyBlob>> for CompilerDiagnostics { fn from(value: Option<ReadOnlyBlob>) -> Self { Self(OptTextBlob::new(     value )) } }
impl From<       ReadOnlyBlob > for CompilerDiagnostics { fn from(value:        ReadOnlyBlob ) -> Self { Self(OptTextBlob::new(Some(value))) } }
