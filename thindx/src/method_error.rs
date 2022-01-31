use crate::*;

use winapi::shared::winerror::{HRESULT, SUCCEEDED};

use std::fmt::{self, Debug, Display, Formatter};



/// An error about some specific method returning an [HRESULT](https://www.hresult.info/)
#[derive(Clone)]
pub struct MethodError(pub(crate) &'static str, pub(crate) ErrorKind);

// TODO: replace MethodError with a generic Error that takes a single `&'static ErrorSite` payload.
// This wil allow:
//  1. a cheaper Error type (narrow ref instead of a fat ref)
//  2. extra metadata (detailed reason, d3d method + thindx method, ...)
// Start with a `method_error!(...)` macro to replace existing `MethodError(...)` tuple construction?

impl MethodError {
    /// Returns an `Err(MethodError(...))` if `!SUCCEEDED(hr)`
    pub fn check(method: &'static str, hr: HRESULT) -> Result<(), Self> {
        if SUCCEEDED(hr) {
            Ok(())
        } else {
            Err(MethodError(method, ErrorKind(hr)))
        }
    }

    pub(crate) fn new(method: &'static str, kind: impl Into<ErrorKind>) -> Self { Self(method, kind.into()) }

    pub(crate) fn method(&self) -> &'static str { self.0 }

    /// Returns the [ErrorKind] of the error
    pub fn kind(&self) -> ErrorKind { self.1 }

    /// Returns the [HRESULT] of the error
    pub fn hresult(&self) -> HRESULT { self.1.0 }

    /// Returns a link in the format of e.g. "<https://www.hresult.info/Search?q=0x80000005>"
    pub fn hresult_info_search_link(&self) -> String { format!("https://www.hresult.info/Search?q=0x{:08x}", self.1.0 as u32) }
}

impl Debug   for MethodError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "MethodError({:?}, {:?})", self.0, self.1) } }
impl Display for MethodError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} failed with HRESULT == {}", self.0, self.1) } }

impl std::error::Error for MethodError {}
impl From<MethodError> for std::io::Error { fn from(err: MethodError) -> Self { std::io::Error::new(std::io::ErrorKind::Other, err) } }

impl PartialEq<Option<MethodError>> for ErrorKind { fn eq(&self, other: &Option<MethodError>) -> bool { Some(*self) == other.as_ref().map(|e| e.kind()) } }
impl PartialEq<ErrorKind> for Option<MethodError> { fn eq(&self, other: &ErrorKind)           -> bool { Some(*other) == self.as_ref().map(|e| e.kind()) } }
impl<O> PartialEq<Result<O, MethodError>> for ErrorKind { fn eq(&self, other: &Result<O, MethodError>) -> bool { Some(*self) == other.as_ref().err().map(|e| e.kind()) } }
impl<O> PartialEq<ErrorKind> for Result<O, MethodError> { fn eq(&self, other: &ErrorKind)              -> bool { Some(*other) == self.as_ref().err().map(|e| e.kind()) } }
