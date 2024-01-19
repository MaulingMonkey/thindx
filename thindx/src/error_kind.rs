use crate::*;

use bytemuck::*;

use winapi::shared::winerror::*;

use winresult::{ErrorCode, HResult, HResultError};

use std::fmt::{self, Debug, Display, Formatter};



// https://learn.microsoft.com/en-us/windows/win32/com/structure-of-com-error-codes
// https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3derr

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/direct3d11/d3d11-graphics-reference-returnvalues)\]
/// HRESULT
///
/// See [thindx::errors](crate::errors) for a list of constants
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ErrorKind(HRESULT);

enumish! { ErrorKind => HRESULT }

impl ErrorKind {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/dmerror/nf-dmerror-make_hresult)\]
    /// MAKE_HRESULT
    ///
    /// ### Arguments
    /// *   `sev`   - the severity of the errors (e.g. `0` for succcess, `1` for failure.)
    /// *   `fac`   - the `FACILITY_*` category of the error (e.g. `FACILITY_WIN32`.)
    /// *   `code`  - the `ERROR_*` code/value (e.g. `ERROR_FILE_NOT_FOUND`)
    pub const fn make_hresult(sev: u32, fac: u32, code: u32) -> Self {
        Self(((sev << 31) | (fac << 16) | (code << 0)) as _)
    }

    /// MAKE_D3DHRESULT
    pub const fn make_d3dhresult(code: u32) -> ErrorKind { ErrorKind::make_hresult(1, _FACD3D, code) }

    /// MAKE_DDHRESULT
    pub const fn make_ddhresult(code: u32) -> ErrorKind { ErrorKind::make_hresult(1, _FACD3D, code) } // Yes, _FACD3D is the same

    /// MAKE_D3DSTATUS
    pub const fn make_d3dstatus(code: u32) -> ErrorKind { ErrorKind::make_hresult(0, _FACD3D, code) }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/dmerror/nf-dmerror-make_hresult)\]
    /// MAKE_HRESULT(1, FACILITY_WIN32, code)
    ///
    /// ### Arguments
    /// *   `code`  - the `ERROR_*` code/value (e.g. `ERROR_FILE_NOT_FOUND`)
    pub const fn from_win32(code: u32) -> Self { Self::make_hresult(1, 7, code) }

    #[allow(missing_docs)] pub const fn to_code    (self) -> Option<ErrorCode> { if (self.0 as u32) < 0x10000 { Some(ErrorCode::from_constant(self.0 as _)) } else { None } }
    #[allow(missing_docs)] pub const fn to_hresult (self) -> Option<HResult  > { if (self.0 as u32) >=0x10000 { Some(HResult  ::from_constant(self.0 as _)) } else { None } }
    #[allow(missing_docs)] pub const fn from_winapi(value: HRESULT) -> Self { Self(value) } // TODO: remove
    #[allow(missing_docs)] pub const fn to_winapi(self) -> HRESULT { self.0 } // TODO: remove
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0 as u32;
        if value < 0x10000 {
            write!(f, "{:?} ({value})", ErrorCode::from(value as u16))
        } else {
            write!(f, "{:?} (0x{value:08X})", HResult::from(value))
        }
    }
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0 as u32;
        if value < 0x10000 {
            write!(f, "{:?} ({value})", ErrorCode::from(value as u16))
        } else {
            write!(f, "{:?} (0x{value:08X})", HResult::from(value))
        }
    }
}

impl std::error::Error for ErrorKind {}

impl From<std::ffi::NulError> for ErrorKind {
    fn from(_: std::ffi::NulError) -> ErrorKind { THINERR::STRING_CONTAINS_NULS.into() }
}

impl From<abistr::InteriorNulError> for ErrorKind {
    fn from(_: abistr::InteriorNulError) -> ErrorKind { THINERR::STRING_CONTAINS_NULS.into() }
}

impl From<ErrorCode     > for ErrorKind { fn from(ec: ErrorCode     ) -> Self { Self(u32::from(ec) as _) } }
impl From<HResultError  > for ErrorKind { fn from(hr: HResultError  ) -> Self { Self(u32::from(hr) as _) } }
//impl From<HResult     > for ErrorKind { fn from(hr: HResult       ) -> Self { Self(u32::from(hr) as _) } }

impl PartialEq<HRESULT> for ErrorKind { fn eq(&self, other: &HRESULT)   -> bool { self.0 == *other } }
impl PartialEq<ErrorKind> for HRESULT { fn eq(&self, other: &ErrorKind)    -> bool { other.0 == *self } }

impl PartialEq<Option<ErrorKind>> for ErrorKind { fn eq(&self, other: &Option<ErrorKind>) -> bool { Some(self) == other.as_ref() } }
impl PartialEq<ErrorKind> for Option<ErrorKind> { fn eq(&self, other: &ErrorKind)         -> bool { Some(other) == self.as_ref() } }

impl<O> PartialEq<Result<O, ErrorKind>> for ErrorKind { fn eq(&self, other: &Result<O, ErrorKind>) -> bool { Some(self) == other.as_ref().err() } }
impl<O> PartialEq<ErrorKind> for Result<O, ErrorKind> { fn eq(&self, other: &ErrorKind)            -> bool { Some(other) == self.as_ref().err() } }

impl PartialEq<ErrorCode> for ErrorKind    { fn eq(&self, other: &ErrorCode     ) -> bool { ErrorKind::from(*self) == ErrorKind::from(*other) } }
impl PartialEq<ErrorKind> for ErrorCode    { fn eq(&self, other: &ErrorKind     ) -> bool { ErrorKind::from(*self) == ErrorKind::from(*other) } }
impl PartialEq<HResultError> for ErrorKind { fn eq(&self, other: &HResultError  ) -> bool { ErrorKind::from(*self) == ErrorKind::from(*other) } }
impl PartialEq<ErrorKind> for HResultError { fn eq(&self, other: &ErrorKind     ) -> bool { ErrorKind::from(*self) == ErrorKind::from(*other) } }

const _FACD3D : u32 = 0x876; // d3d9helper.h

//#cpp2rust HRESULT         = ErrorKind
//#cpp2rust MAKE_HRESULT    = ErrorKind::make_hresult

//#cpp2rust MAKE_D3DSTATUS  = ErrorKind::make_d3dstatus
//#cpp2rust MAKE_D3DHRESULT = ErrorKind::make_d3dhresult
//#cpp2rust MAKE_DDHRESULT  = ErrorKind::make_ddhresult
