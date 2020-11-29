use crate::*;

use winapi::ctypes::c_char;

use std::borrow::Cow;
use std::ffi::*;
use std::ptr::*;



/// Converts `self` into something that implements [AsCStr]
pub trait TryIntoAsCStr {
    type Target : AsCStr;
    fn try_into(self) -> Result<Self::Target, ErrorKind>;
}

impl<T: AsCStr> TryIntoAsCStr for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, ErrorKind> { Ok(self) }
}

impl<'s> TryIntoAsCStr for &'s str {
    type Target = ImplCStr<'s>;
    fn try_into(self) -> Result<Self::Target, ErrorKind> {
        if self.contains('\0') {
            Err(ErrorKind::STRING_CONTAINS_NULS)
        } else {
            Ok(ImplCStr(Cow::Owned(self.bytes().chain(Some(0)).collect())))
        }
    }
}

impl TryIntoAsCStr for String {
    type Target = ImplCStr<'static>;
    fn try_into(mut self) -> Result<Self::Target, ErrorKind> {
        if self.contains('\0') {
            Err(ErrorKind::STRING_CONTAINS_NULS)
        } else {
            self.push('\0');
            Ok(ImplCStr(Cow::Owned(self.into_bytes())))
        }
    }
}



/// Converts `self` into something that implements [AsOptCStr]
pub trait TryIntoAsOptCStr {
    type Target : AsOptCStr;
    fn try_into(self) -> Result<Self::Target, ErrorKind>;
}

impl TryIntoAsOptCStr for () {
    type Target = ();
    fn try_into(self) -> Result<Self::Target, ErrorKind> { Ok(()) }
}

impl<T: TryIntoAsCStr> TryIntoAsOptCStr for T {
    type Target = T::Target;
    fn try_into(self) -> Result<Self::Target, ErrorKind> { TryIntoAsCStr::try_into(self) }
}

impl<T: TryIntoAsCStr> TryIntoAsOptCStr for Option<T> {
    type Target = Option<T::Target>;
    fn try_into(self) -> Result<Self::Target, ErrorKind> {
        match self {
            None => Ok(None),
            Some(x) => Ok(Some(x.try_into()?))
        }
    }
}



/// Treat `self` as a C-style string
///
/// ### Safety
///
/// *   You promise the returned pointer is valid and points to a `\0`-terminated string
pub unsafe trait AsCStr {
    fn as_cstr(&self) -> *const c_char;
}

unsafe impl AsCStr for &'_ CStr {
    fn as_cstr(&self) -> *const c_char { self.as_ptr() }
}

unsafe impl AsCStr for CString {
    fn as_cstr(&self) -> *const c_char { self.as_ptr() }
}



/// Treat `self` as a C-style string
///
/// ### Safety
///
/// *   You promise the returned pointer is null, or points to a valid `\0`-terminated string
pub unsafe trait AsOptCStr {
    fn as_opt_cstr(&self) -> *const c_char;
}

unsafe impl AsOptCStr for () {
    fn as_opt_cstr(&self) -> *const c_char { null() }
}

unsafe impl<T: AsCStr> AsOptCStr for T {
    fn as_opt_cstr(&self) -> *const c_char { self.as_cstr() }
}

unsafe impl<T: AsCStr> AsOptCStr for Option<T> {
    fn as_opt_cstr(&self) -> *const c_char { self.as_ref().map_or(null(), |s| s.as_cstr()) }
}



#[doc(hidden)] pub struct ImplCStr<'c>(Cow<'c, [u8]>);
// Invariants: self.0 must be \0 terminated

unsafe impl AsCStr for ImplCStr<'_> {
    fn as_cstr(&self) -> *const c_char { self.0.as_ptr().cast() }
}
