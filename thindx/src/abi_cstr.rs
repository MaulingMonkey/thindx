use std::borrow::Cow;
use std::ffi::*;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::str::Utf8Error;



/// Option\<&[AbiCStr]\> is meant to be ABI compatible with `*const c_char`
pub struct AbiCStr(());

impl AbiCStr {
    /// Convert a raw C-string into an &[AbiCStr].  Note that the lifetime of the returned reference is unbounded!
    ///
    /// ### Safety
    /// *   `ptr` cannot be null
    /// *   `ptr` must point to a `\0`-terminated C string
    /// *   The underlying C-string cannot change for the duration of the &[AbiCStr]'s lifetime.
    /// *   It is very very very easy to accidentally extend &[AbiCStr]'s lifetime too far - be careful!
    pub unsafe fn from_ptr_unbounded<'unbounded>(ptr: *const c_char) -> &'unbounded AbiCStr {
        &*(ptr as *const Self)
    }

    /// Treat `self` as a `\0`-terminated C string.
    pub fn as_ptr(&self) -> *const c_char {
        (self as *const Self) as *const c_char
    }

    /// Convert a raw slice of bytes to a &[AbiCStr].  `bytes` should end with `\0` but contain no interior `\0`s otherwise.
    pub fn from_bytes_with_nul(bytes: &[u8]) -> Result<&AbiCStr, FromBytesWithNulError> {
        CStr::from_bytes_with_nul(bytes).map(<&AbiCStr>::from)
    }

    /// Convert a raw slice of bytes to a &[AbiCStr].  The resulting string will be terminated at the first `\0` in `bytes`.
    ///
    /// ### Safety
    /// *   `bytes` must contain at least one `\0`.
    pub unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &AbiCStr {
        debug_assert!(bytes.contains(&0));
        AbiCStr::from_ptr_unbounded(bytes.as_ptr() as *const _)
    }

    /// Convert self to a [std::ffi::CStr].  O(n) to find the terminal `\0`.
    pub fn to_cstr(&self) -> &CStr                  { unsafe { CStr::from_ptr(self.as_ptr()) } }

    /// Convert self to a &\[[u8]\] slice, **excluding** the terminal `\0`.  O(n) to find the terminal `\0`.
    pub fn to_bytes(&self) -> &[u8]                 { self.to_cstr().to_bytes() }

    /// Convert self to a &\[[u8]\] slice, including the terminal `\0`.  O(n) to find the terminal `\0`.
    pub fn to_bytes_with_nul(&self) -> &[u8]        { self.to_cstr().to_bytes_with_nul() }

    /// Convert self to a &[str].  O(n) to find the terminal `\0` and validate UTF8.
    pub fn to_str(&self) -> Result<&str, Utf8Error> { self.to_cstr().to_str() }

    /// Convert self to a [Cow]\<[str]\>.  O(n) to find the terminal `\0` and validate, and to convert UTF8ish data to UTF8 if necesssary.
    pub fn to_string_lossy(&self) -> Cow<'_, str>   { self.to_cstr().to_string_lossy() }
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug::fmt(self.to_cstr(), f) }
}

impl<'s> From<&'s AbiCStr> for &'s CStr {
    fn from(s: &'s AbiCStr) -> Self { s.to_cstr() }
}

impl<'s> From<&'s CStr> for &'s AbiCStr {
    fn from(s: &'s CStr) -> Self { unsafe { AbiCStr::from_ptr_unbounded(s.as_ptr()) } }
}

#[test] fn layout() {
    use winapi::ctypes::c_char;
    use std::mem::*;

    assert_eq!(align_of::<       &AbiCStr >(), align_of::<*const c_char>());
    assert_eq!( size_of::<       &AbiCStr >(),  size_of::<*const c_char>());
    assert_eq!(align_of::<Option<&AbiCStr>>(), align_of::<*const c_char>());
    assert_eq!( size_of::<Option<&AbiCStr>>(),  size_of::<*const c_char>());
}



/// ABI compatible with `*const c_char`, treats null as "".
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ConstCStrPtrNullIsEmpty<'s> {
    ptr:        *const c_char,
    phantom:    PhantomData<&'s c_char>,
}

impl<'s> ConstCStrPtrNullIsEmpty<'s> {
    /// Treat `self` as a raw C string
    pub fn as_ptr(&self) -> *const c_char {
        self.ptr
    }

    /// Treat `self` as a [std::ffi::CStr].  O(n) to find the terminal `\0`.
    pub fn to_cstr(&self) -> &'s CStr {
        if self.ptr.is_null() {
            unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") }
        } else {
            unsafe { CStr::from_ptr(self.ptr) }
        }
    }

    /// Convert self to a &\[[u8]\] slice, **excluding** the terminal `\0`.  O(n) to find the terminal `\0`.
    pub fn to_bytes(&self) -> &[u8]                 { self.to_cstr().to_bytes() }

    /// Convert self to a &\[[u8]\] slice, including the terminal `\0`.  O(n) to find the terminal `\0`.
    pub fn to_bytes_with_nul(&self) -> &[u8]        { self.to_cstr().to_bytes_with_nul() }

    /// Convert self to a &[str].  O(n) to find the terminal `\0` and validate UTF8.
    pub fn to_str(&self) -> Result<&str, Utf8Error> { self.to_cstr().to_str() }

    /// Convert self to a [Cow]\<[str]\>.  O(n) to find the terminal `\0` and validate, and to convert UTF8ish data to UTF8 if necesssary.
    pub fn to_string_lossy(&self) -> Cow<'_, str>   { self.to_cstr().to_string_lossy() }
}

impl Debug for ConstCStrPtrNullIsEmpty<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug::fmt(self.to_cstr(), f) }
}

impl Default for ConstCStrPtrNullIsEmpty<'_> {
    fn default() -> Self { Self { ptr: b"\0".as_ptr().cast(), phantom: PhantomData } }
}

impl<'s> From<ConstCStrPtrNullIsEmpty<'s>> for &'s CStr {
    fn from(s: ConstCStrPtrNullIsEmpty<'s>) -> Self { s.to_cstr() }
}
