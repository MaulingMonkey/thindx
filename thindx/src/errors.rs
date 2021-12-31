//! [ErrorKind] values

#![allow(overflowing_literals)] // ErrorKind is signed for some reason
#![allow(non_snake_case)]

use crate::*;
use winapi::shared::winerror::*;

// https://docs.microsoft.com/en-us/windows/win32/direct3d11/d3d11-graphics-reference-returnvalues

// https://docs.microsoft.com/en-us/windows/win32/com/structure-of-com-error-codes
// #        Facility            Desc
// 0        FACILITY_NULL       For broadly applicable common status codes such as S_OK.
// 1        FACILITY_RPC        For status codes returned from remote procedure calls.
// 2        FACILITY_DISPATCH   For late-binding IDispatch interface errors.
// 3        FACILITY_STORAGE    For status codes returned from IStorage or IStream method calls relating to structured storage. Status codes whose code (lower 16 bits) value is in the range of MS-DOS error codes (that is, less than 256) have the same meaning as the corresponding MS-DOS error.
// 4        FACILITY_ITF        For most status codes returned from interface methods. The actual meaning of the error is defined by the interface. That is, two HRESULTs with exactly the same 32-bit value returned from two different interfaces might have different meanings.
// 7        FACILITY_WIN32      Used to provide a means of handling error codes from functions in the Windows API as an HRESULT. Error codes in 16-bit OLE that duplicated system error codes have also been changed to FACILITY_WIN32.
// 8        FACILITY_WINDOWS    Used for additional error codes from Microsoft-defined interfaces.
//  0x876   _FACD3D             Direct3D
//  0x879                       Direct3D 10
//  0x87A                       DXGI
//  0x87B                       DXGI DDI
//  0x87C                       Direct3D 11
//  0x898                       DirectWrite
//  0x899                       Direct2D
// 0xA73D                       Thin3D

/// `0xA73D....` • ThinDX [ErrorKind]s
pub mod THINERR {
    use super::*;

    /// `0xA73DC001`    This version of `d3dcompiler_##.dll` doesn't support this fn
    pub const MISSING_DLL_EXPORT : ErrorKind = ErrorKind(0xA73DC001);

    /// `0xA73DC002`    Slice length exceeded some kind of length limit (typically a conversion to a 32-bit length, or
    ///                 an extra cap introduced by thindx to avoid undefined behavior from allocation size overflows.)
    pub const SLICE_TOO_LARGE : ErrorKind = ErrorKind(0xA73DC002);

    /// `0xA73DC003`    String contains unexpected internal `\0`s when being passed to a function taking C-style `\0`-*terminated* strings.
    pub const STRING_CONTAINS_NULS : ErrorKind = ErrorKind(0xA73DC003);

    /// `0xA73DC004`    Bytecode is invalid (bad header, invalid checksum, wrong length, etc.)
    pub const INVALID_BYTECODE : ErrorKind = ErrorKind(0xA73DC004);
}

/// `0x887C....` • Direct3D 11 [ErrorKind]s
pub mod D3D11_ERROR {
    use super::*;

    /// The file was not found.
    pub const FILE_NOT_FOUND                                : ErrorKind = ErrorKind(D3D11_ERROR_FILE_NOT_FOUND);

    /// There are too many unique instances of a particular type of state object.
    pub const TOO_MANY_UNIQUE_STATE_OBJECTS                 : ErrorKind = ErrorKind(D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS);

    /// There are too many unique instances of a particular type of view object.
    pub const TOO_MANY_UNIQUE_VIEW_OBJECTS                  : ErrorKind = ErrorKind(D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS);

    /// The first call to [ID3D11DeviceContext::Map] after either [ID3D11Device::CreateDeferredContext] or [ID3D11DeviceContext::FinishCommandList] per Resource was not D3D11_MAP_WRITE_DISCARD.
    ///
    /// [ID3D11DeviceContext::Map]:                 https://docs.microsoft.com/en-us/windows/desktop/api/D3D11/nf-d3d11-id3d11devicecontext-map
    /// [ID3D11Device::CreateDeferredContext]:      https://docs.microsoft.com/en-us/windows/desktop/api/D3D11/nf-d3d11-id3d11device-createdeferredcontext
    /// [ID3D11DeviceContext::FinishCommandList]:   https://docs.microsoft.com/en-us/windows/desktop/api/D3D11/nf-d3d11-id3d11devicecontext-finishcommandlist
    pub const DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD  : ErrorKind = ErrorKind(D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD);
}

const D3DERR_INVALIDCALL        : HRESULT = MAKE_D3DHRESULT(2156);
const D3DERR_WASSTILLDRAWING    : HRESULT = MAKE_D3DHRESULT(540);

/// `0x8876....` • Direct3D / Direct3D9 [ErrorKind]s
pub mod D3DERR {
    use super::*;

    /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
    pub const INVALIDCALL               : ErrorKind = ErrorKind(D3DERR_INVALIDCALL);

    /// The previous blit operation that is transferring information to or from this surface is incomplete.
    pub const WASSTILLDRAWING           : ErrorKind = ErrorKind(D3DERR_WASSTILLDRAWING);
}

/// `0x887A....` • DXGI [ErrorKind]s
pub mod DXGI_ERROR {
    use super::*;

    /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
    pub const INVALID_CALL              : ErrorKind = ErrorKind(DXGI_ERROR_INVALID_CALL);

    /// The previous blit operation that is transferring information to or from this surface is incomplete.
    pub const WAS_STILL_DRAWING         : ErrorKind = ErrorKind(DXGI_ERROR_WAS_STILL_DRAWING);
}

/// `0x8000....` • General [ErrorKind]s<br>
/// `0x8007....`
pub mod E {
    use super::*;

    /// Attempted to create a device with the debug layer enabled and the layer is not installed.
    pub const FAIL                      : ErrorKind = ErrorKind(E_FAIL);

    /// An invalid parameter was passed to the returning function.
    pub const INVALIDARG                : ErrorKind = ErrorKind(E_INVALIDARG);

    /// Direct3D could not allocate sufficient memory to complete the call.
    pub const OUTOFMEMORY               : ErrorKind = ErrorKind(E_OUTOFMEMORY);

    /// The method call isn't implemented with the passed parameter combination.
    pub const NOTIMPL                   : ErrorKind = ErrorKind(E_NOTIMPL);

    /// Access is denied.
    pub const ACCESSDENIED              : ErrorKind = ErrorKind(E_ACCESSDENIED);

    /// The system cannot find the path specified.
    pub const E_PATH_NOT_FOUND          : ErrorKind = ErrorKind(E_PATH_NOT_FOUND);
}

/// `0x8000....` • General [ErrorKind]s<br>
/// `0x8007....`
pub mod ERROR {
    /// A file was not found
    pub const FILE_NOT_FOUND            : ErrorKind = ErrorKind(ERROR_FILE_NOT_FOUND);

    /// Access is denied.
    pub const ACCESS_DENIED             : ErrorKind = ErrorKind(ERROR_ACCESS_DENIED);

    /// The system cannot find the path specified.
    pub const PATH_NOT_FOUND            : ErrorKind = ErrorKind(ERROR_PATH_NOT_FOUND);

    /// The file exists.
    pub const FILE_EXISTS               : ErrorKind = ErrorKind(ERROR_FILE_EXISTS);
}

/// `0x0000....` • Success "[ErrorKind]"s
pub mod S {
    use super::*;

    /// No error occurred.
    pub const OK                        : ErrorKind = ErrorKind(S_OK);

    /// Alternate success value, indicating a successful but nonstandard completion (the precise meaning depends on context).
    pub const FALSE                     : ErrorKind = ErrorKind(S_FALSE);
}



// d3d9helper.h
const _FACD3D : u32 = 0x876;
const fn MAKE_D3DHRESULT(code: u32) -> HRESULT { MAKE_HRESULT(1, _FACD3D, code) }
//const fn MAKE_DDHRESULT(code: u32)  -> HRESULT { MAKE_HRESULT(1, _FACD3D, code) } // Yes, _FACD3D is the same
//const fn MAKE_D3DSTATUS (code: u32) -> HRESULT { MAKE_HRESULT(0, _FACD3D, code) }
const fn MAKE_HRESULT(sev: u32, fac: u32, code: u32) -> HRESULT { (sev << 31 | fac << 16 | code) as HRESULT }
