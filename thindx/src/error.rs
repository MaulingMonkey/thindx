use crate::*;
use crate::error_macros::FnContext;

use winapi::shared::winerror::{HRESULT, SUCCEEDED};

use std::fmt::{self, Debug, Display, Formatter};



/// An error about some specific method returning an [HRESULT](https://www.hresult.info/)
#[derive(Clone)]
pub struct Error(pub(crate) &'static FnContext, pub(crate) ErrorKind);

// TODO: replace Error with a generic Error that takes a single `&'static ErrorSite` payload.
// This wil allow:
//  1. a cheaper Error type (narrow ref instead of a fat ref)
//  2. extra metadata (detailed reason, d3d method + thindx method, ...)
// Start with a `method_error!(...)` macro to replace existing `Error(...)` tuple construction?

impl Error {
    /// Returns an `Err(Error(...))` if `!SUCCEEDED(hr)`
    pub(crate) fn check(ctx: &'static FnContext, hr: HRESULT) -> Result<(), Self> {
        if SUCCEEDED(hr) {
            Ok(())
        } else {
            Err(Error(ctx, ErrorKind(hr)))
        }
    }

    pub(crate) fn method(&self) -> &'static str { self.0.directx_method.unwrap_or(self.0.thindx_method) }

    /// Returns the [ErrorKind] of the error
    pub fn kind(&self) -> ErrorKind { self.1 }

    /// Returns the [HRESULT] of the error
    pub fn hresult(&self) -> HRESULT { self.1.0 }
}

impl Debug   for Error { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "Error({:?}, {:?})", self.method(), self.1) } }
impl Display for Error { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} failed with HRESULT == {}", self.method(), self.1) } }

impl std::error::Error for Error {}
impl From<Error> for std::io::Error { fn from(err: Error) -> Self { std::io::Error::new(std::io::ErrorKind::Other, err) } }

impl PartialEq<Option<Error>> for ErrorKind { fn eq(&self, other: &Option<Error>) -> bool { Some(*self) == other.as_ref().map(|e| e.kind()) } }
impl PartialEq<ErrorKind> for Option<Error> { fn eq(&self, other: &ErrorKind)           -> bool { Some(*other) == self.as_ref().map(|e| e.kind()) } }
impl<O> PartialEq<Result<O, Error>> for ErrorKind { fn eq(&self, other: &Result<O, Error>) -> bool { Some(*self) == other.as_ref().err().map(|e| e.kind()) } }
impl<O> PartialEq<ErrorKind> for Result<O, Error> { fn eq(&self, other: &ErrorKind)              -> bool { Some(*other) == self.as_ref().err().map(|e| e.kind()) } }
