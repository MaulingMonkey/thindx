//! [S],
//! [D3D],
//! [E],
//! [ERROR],
//! [D3DERR],
//! [D3DXERR],
//! [D3D11],
//! [DXGI],
//! and [THINERR]
//! [ErrorKind] values<br>
//! **NOTE:** Imported into crate root, no need to prefix `errors::`...
//!
//! DirectX APIs return a mixture of:
//! *   Non-[HRESULT] S_\* and ERROR_\* codes
//! *   MAKE_HRESULT(1, FACILITY_WIN32, ...)ified ERROR_\* codes
//! *   Other [HRESULT]s with proper names
//!
//! Sometimes from the same individual function!
//! Sanitizing this muddle for the general case is beyond the scope of thindx.<br>
//! As such, [ErrorKind] awkwardly muddles these all together too.
//!
//! | [HRESULT]    | Facility           | Desc  |
//! | ------------:| ------------------ | ----- |
//! | `0x...0....` | FACILITY_NULL      | For broadly applicable common status codes such as S_OK.
//! | `0x...1....` | FACILITY_RPC       | For status codes returned from remote procedure calls.
//! | `0x...2....` | FACILITY_DISPATCH  | For late-binding IDispatch interface errors.
//! | `0x...3....` | FACILITY_STORAGE   | Returned from IStorage or IStream method calls relating to structured storage.
//! | `0x...4....` | FACILITY_ITF       | For most status codes returned from interface methods.
//! | `0x...7....` | FACILITY_WIN32     | Windows ERROR_\* codes packaged as an HRESULT.
//! | `0x...8....` | FACILITY_WINDOWS   | Used for additional error codes from Microsoft-defined interfaces.
// | `0x27D8....` |                    | ThinDX Success Codes (2=Customer Bit, 7D8 = TDX)
//! | `0xA7D8....` |                    | ThinDX Error Codes (A=2\|8=Customer\|Error Bits, 7D8 = TDX = ThinDX)
//! | `0x.876....` | _FACD3D            | Direct3D (9) / DirectDraw
//! | `0x.879....` |                    | Direct3D 10
//! | `0x.87A....` |                    | DXGI
//! | `0x.87B....` |                    | DXGI DDI
//! | `0x.87C....` |                    | Direct3D 11
//! | `0x.898....` |                    | DirectWrite
//! | `0x.899....` |                    | Direct2D
//!
//! ### See Also
//! *   <https://www.hresult.info/>
//! *   <https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes#system-error-codes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/direct3d11/d3d11-graphics-reference-returnvalues>
//! *   <https://learn.microsoft.com/en-us/windows/win32/com/structure-of-com-error-codes>
//!
//! [HRESULT]:  https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a

#![allow(overflowing_literals)] // ErrorKind is signed for some reason
#![allow(non_snake_case)]
// TODO: Cleanup formatting etc.

#[allow(unused_imports)] use crate::*;
#[allow(unused_imports)] use crate::d3d9::*;
use winresult::HResultError;



/// `0xA7D8....` • **T**hin**DX** [ErrorKind]s
///
/// *   `0xA.......`  - **S**everity and **C**ustomer bits for [HRESULT](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)s
/// *   `0x.7D8....`  - **T**hin **DX** error codes
pub mod THINERR {
    use super::*;

    // General errors

    /// `0xA7D80000`    A nonspecific error of some sort occured.  This should generally only be used when DirectX returned some kind of error or unexpected null pointer, without actually bothering to say what the error was.
    pub const NONSPECIFIC : HResultError = HResultError::from_constant(0xA7D80000);

    // `0xA7D80001` was SLICE_OVERFLOW, prefer SLICE_TOO_LARGE

    /// `0xA7D80002`    Resource belonging to one [Device] was passed to a different [Device].  To avoid undefined behavior, DirectX was not called.
    pub const DEVICE_MISMATCH   : HResultError = HResultError::from_constant(0xA7D80002);

    /// `0xA7D80003`    Large allocation size was requested.  To avoid undefined behavior from arithmetic overflows, DirectX was not called.
    pub const ALLOC_OVERFLOW    : HResultError = HResultError::from_constant(0xA7D80003);

    /// `0xA7D80004`    A structure contained some kind of field such as `dwSize` or `iType` that was invalid.
    pub const INVALID_STRUCT_FIELD : HResultError = HResultError::from_constant(0xA7D80004);

    /// `0xA7D80005`    This version of the DLL doesn't support this fn
    pub const MISSING_DLL_EXPORT : HResultError = HResultError::from_constant(0xA7D80005);

    /// `0xA7D80006`    Slice length exceeded some kind of length limit (typically a conversion to a 32-bit length, or
    ///                 an extra cap introduced by thindx to avoid undefined behavior from allocation size overflows.)
    pub const SLICE_TOO_LARGE : HResultError = HResultError::from_constant(0xA7D80006);

    /// `0xA7D80007`    String contains unexpected internal `\0`s when being passed to a function taking C-style `\0`-*terminated* strings.
    pub const STRING_CONTAINS_NULS : HResultError = HResultError::from_constant(0xA7D80007);

    /// `0xA7D80008`    Bytecode is invalid (bad header, invalid checksum, wrong length, etc.)
    pub const INVALID_BYTECODE : HResultError = HResultError::from_constant(0xA7D80008);
}

/// `0x887C....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d11/d3d11-graphics-reference-returnvalues)\] • Direct3D 11 [ErrorKind](crate::ErrorKind)s
pub use winresult::D3D11;

/// `0x8876....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3derr)\] • Direct3D / Direct3D9 [ErrorKind](crate::ErrorKind)s
pub use winresult::D3DERR;

/// `0x8876....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dxerr)\] • D3DX [ErrorKind](crate::ErrorKind)s
///
/// Some of these can be returned by Direct3D itself
pub use winresult::D3DXERR;

/// `0x887A....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-error)\] • DXGI [ErrorKind](crate::ErrorKind)s
pub use winresult::DXGI;

/// `0x8000....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/learnwin32/error-handling-in-com)\] • General [ErrorKind](crate::ErrorKind)s<br>
/// `0x8007....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/learnwin32/error-handling-in-com)\] • Win32/COM [ErrorKind](crate::ErrorKind)s
///
/// Errors that aren't part of the D3DERR_\* family, but might still be returned by DirectX API calls.
pub use winresult::E;

/// `0x0000....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-)\] • Non-hresult [ErrorKind](crate::ErrorKind)s
pub use winresult::ERROR;

/// `0x0000....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/learnwin32/error-handling-in-com)\] • Win32/COM success "[ErrorKind](crate::ErrorKind)"s<br>
/// `0x0876....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3derr)\] • Success "[ErrorKind](crate::ErrorKind)"s
pub use winresult::S;

/// `0x0876....` • \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3derr)\] • Success "[ErrorKind](crate::ErrorKind)"s
pub mod D3D {
    pub use winresult::D3D::*;
    pub use winresult::D3DOK::NOAUTOGEN as OK_NOAUTOGEN; // TODO: eliminate in favor of fixing winresult
}

//#cpp2rust S_NOT_RESIDENT                      = S::NOT_RESIDENT
//#cpp2rust S_PRESENT_MODE_CHANGED              = S::PRESENT_MODE_CHANGED
//#cpp2rust S_PRESENT_OCCLUDED                  = S::PRESENT_OCCLUDED
//#cpp2rust S_RESIDENT_IN_SHARED_MEMORY         = S::RESIDENT_IN_SHARED_MEMORY

//#cpp2rust D3D_OK                              = D3D::OK
//#cpp2rust D3DOK_NOAUTOGEN                     = D3D::OK_NOAUTOGEN

//#cpp2rust D3DERR_CANNOTPROTECTCONTENT         = D3DERR::CANNOTPROTECTCONTENT
//#cpp2rust D3DERR_CONFLICTINGRENDERSTATE       = D3DERR::CONFLICTINGRENDERSTATE
//#cpp2rust D3DERR_CONFLICTINGTEXTUREFILTER     = D3DERR::CONFLICTINGTEXTUREFILTER
//#cpp2rust D3DERR_CONFLICTINGTEXTUREPALETTE    = D3DERR::CONFLICTINGTEXTUREPALETTE
//#cpp2rust D3DERR_DEVICEHUNG                   = D3DERR::DEVICEHUNG
//#cpp2rust D3DERR_DEVICELOST                   = D3DERR::DEVICELOST
//#cpp2rust D3DERR_DEVICENOTRESET               = D3DERR::DEVICENOTRESET
//#cpp2rust D3DERR_DEVICEREMOVED                = D3DERR::DEVICEREMOVED
//#cpp2rust D3DERR_DRIVERINTERNALERROR          = D3DERR::DRIVERINTERNALERROR
//#cpp2rust D3DERR_DRIVERINVALIDCALL            = D3DERR::DRIVERINVALIDCALL
//#cpp2rust D3DERR_INVALIDCALL                  = D3DERR::INVALIDCALL
//#cpp2rust D3DERR_INVALIDDEVICE                = D3DERR::INVALIDDEVICE
//#cpp2rust D3DERR_MOREDATA                     = D3DERR::MOREDATA
//#cpp2rust D3DERR_NOTAVAILABLE                 = D3DERR::NOTAVAILABLE
//#cpp2rust D3DERR_NOTFOUND                     = D3DERR::NOTFOUND
//#cpp2rust D3DERR_OUTOFVIDEOMEMORY             = D3DERR::OUTOFVIDEOMEMORY
//#cpp2rust D3DERR_PRESENT_STATISTICS_DISJOINT  = D3DERR::PRESENT_STATISTICS_DISJOINT
//#cpp2rust D3DERR_TOOMANYOPERATIONS            = D3DERR::TOOMANYOPERATIONS
//#cpp2rust D3DERR_UNSUPPORTEDALPHAARG          = D3DERR::UNSUPPORTEDALPHAARG
//#cpp2rust D3DERR_UNSUPPORTEDALPHAOPERATION    = D3DERR::UNSUPPORTEDALPHAOPERATION
//#cpp2rust D3DERR_UNSUPPORTEDCOLORARG          = D3DERR::UNSUPPORTEDCOLORARG
//#cpp2rust D3DERR_UNSUPPORTEDCOLOROPERATION    = D3DERR::UNSUPPORTEDCOLOROPERATION
//#cpp2rust D3DERR_UNSUPPORTEDCRYPTO            = D3DERR::UNSUPPORTEDCRYPTO
//#cpp2rust D3DERR_UNSUPPORTEDFACTORVALUE       = D3DERR::UNSUPPORTEDFACTORVALUE
//#cpp2rust D3DERR_UNSUPPORTEDOVERLAY           = D3DERR::UNSUPPORTEDOVERLAY
//#cpp2rust D3DERR_UNSUPPORTEDOVERLAYFORMAT     = D3DERR::UNSUPPORTEDOVERLAYFORMAT
//#cpp2rust D3DERR_UNSUPPORTEDTEXTUREFILTER     = D3DERR::UNSUPPORTEDTEXTUREFILTER
//#cpp2rust D3DERR_WASSTILLDRAWING              = D3DERR::WASSTILLDRAWING
//#cpp2rust D3DERR_WRONGTEXTUREFORMAT           = D3DERR::WRONGTEXTUREFORMAT
