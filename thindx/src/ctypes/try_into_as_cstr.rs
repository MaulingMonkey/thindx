use crate::*;
use crate::ctypes::*;

use winapi::ctypes::c_char;

use std::ffi::*;
use std::ptr::*;



/// Converts `self` into something that implements [AsCStr]
pub trait TryIntoAsCStr {
    /// The temporary type that can be treated as a C-string.
    type Target : AsCStr;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
    fn try_into(self) -> Result<Self::Target, ErrorKind>;
}

impl<T: AsCStr> TryIntoAsCStr for T {
    type Target = T;
    fn try_into(self) -> Result<Self::Target, ErrorKind> { Ok(self) }
}

impl<'s> TryIntoAsCStr for &'s str {
    type Target = CString;
    fn try_into(self) -> Result<Self::Target, ErrorKind> {
        CString::new(self).map_err(|_| THINERR::STRING_CONTAINS_NULS)
    }
}

impl TryIntoAsCStr for String {
    type Target = CString;
    fn try_into(self) -> Result<Self::Target, ErrorKind> {
        CString::new(self).map_err(|_| THINERR::STRING_CONTAINS_NULS)
    }
}



/// Converts `self` into something that implements [AsOptCStr]
pub trait TryIntoAsOptCStr {
    /// The temporary type that can be treated as an [Option]al C-string.
    type Target : AsOptCStr;

    /// Attempt to convert to [Self::Target].  May fail if `self` contains `\0`s.
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
    /// Returns a `\0`-terminated C string
    fn as_cstr(&self) -> *const c_char;
}

unsafe impl AsCStr for &'_ AbiCStr {
    fn as_cstr(&self) -> *const c_char { self.as_ptr() }
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
    /// Returns a `\0`-terminated C string, or [null]\(\).
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
