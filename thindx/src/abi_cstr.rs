use std::borrow::Cow;
use std::ffi::*;
use std::fmt::{self, Debug, Formatter};
use std::os::raw::c_char;
use std::str::Utf8Error;



pub struct AbiCStr(());

impl AbiCStr {
    pub unsafe fn from_ptr_unbounded<'unbounded>(ptr: *const c_char) -> &'unbounded AbiCStr {
        &*(ptr as *const Self)
    }

    pub fn as_ptr(&self) -> *const c_char {
        (self as *const Self) as *const c_char
    }

    pub fn from_bytes_with_nul(bytes: &[u8]) -> Result<&AbiCStr, FromBytesWithNulError> {
        CStr::from_bytes_with_nul(bytes).map(<&AbiCStr>::from)
    }

    pub unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &AbiCStr {
        AbiCStr::from_ptr_unbounded(bytes.as_ptr() as *const _)
    }

    pub fn to_bytes(&self) -> &[u8]                 { <&CStr>::from(self).to_bytes() }
    pub fn to_bytes_with_nul(&self) -> &[u8]        { <&CStr>::from(self).to_bytes_with_nul() }
    pub fn to_str(&self) -> Result<&str, Utf8Error> { <&CStr>::from(self).to_str() }
    pub fn to_string_lossy(&self) -> Cow<'_, str>   { <&CStr>::from(self).to_string_lossy() }
}

#[doc(hidden)] impl AbiCStr {
    /// **❌ Do not use this function directly, it is unsound ❌**
    ///
    /// **Use [from_bytes_with_nul_unchecked](Self::from_bytes_with_nul_unchecked) in an `unsafe { .... }` block instead.**
    ///
    /// It exists only to support allowing the use of the [abicstr!] macro (which uses this in a sound manner) in `#![forbid(unsafe_code)]` codebases.
    #[allow(non_snake_case)]
    pub fn _xxx_unsafe_unsound_xxx__do_not_call_this_directly__use_from_bytes_with_nul_unchecked_instead(bytes: &[u8]) -> &AbiCStr {
        unsafe { Self::from_bytes_with_nul_unchecked(bytes) }
    }
}

/// Creates a `\0`-terminated &'static [AbiCStr].
///
/// ⚠️ Interior nulls will effectively terminate the string early.  Do not use them.
/// Eventually, they are likely to generate at least a warning, or even a hard error.
#[macro_export] macro_rules! cstr {
    ($literal:literal) => {
        $crate::AbiCStr::_xxx_unsafe_unsound_xxx__do_not_call_this_directly__use_from_bytes_with_nul_unchecked_instead(concat!($literal, "\0").as_bytes())
    };
}

impl Debug for AbiCStr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(<&CStr>::from(self), f)
    }
}

impl<'s> From<&'s AbiCStr> for &'s CStr {
    fn from(s: &'s AbiCStr) -> Self {
        unsafe { CStr::from_ptr(s.as_ptr()) }
    }
}

impl<'s> From<&'s CStr> for &'s AbiCStr {
    fn from(s: &'s CStr) -> Self {
        unsafe { AbiCStr::from_ptr_unbounded(s.as_ptr()) }
    }
}

#[test] fn layout() {
    use winapi::ctypes::c_char;
    use std::mem::*;

    assert_eq!(align_of::<       &AbiCStr >(), align_of::<*const c_char>());
    assert_eq!( size_of::<       &AbiCStr >(),  size_of::<*const c_char>());
    assert_eq!(align_of::<Option<&AbiCStr>>(), align_of::<*const c_char>());
    assert_eq!( size_of::<Option<&AbiCStr>>(),  size_of::<*const c_char>());
}
