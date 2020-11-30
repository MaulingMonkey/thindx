//! [ErrorKind] values

use crate::*;
use winapi::shared::winerror::*;

// https://docs.microsoft.com/en-us/windows/win32/direct3d11/d3d11-graphics-reference-returnvalues



/// Thin* errors
#[allow(non_camel_case_types)] pub struct THINERR(());
#[allow(overflowing_literals)] impl THINERR {
    /// `0xA73DC001`    This version of `d3dcompiler_##.dll` doesn't support this fn
    pub const MISSING_DLL_EXPORT : ErrorKind = ErrorKind(0xA73DC001);

    /// `0xA73DC002`    Slice length exceeded some kind of length limit (typically a conversion to a 32-bit length, or
    ///                 an extra cap introduced by thin3dcompiler to avoid undefined behavior from allocation size overflows.)
    pub const SLICE_TOO_LARGE : ErrorKind = ErrorKind(0xA73DC002);

    /// `0xA73DC003`    String contains unexpected internal `\0`s when being passed to a function taking C-style `\0`-*terminated* strings.
    pub const STRING_CONTAINS_NULS : ErrorKind = ErrorKind(0xA73DC003);
}

/// Direct3D 11 errors
#[allow(non_camel_case_types)] pub struct D3D11_ERROR(()); impl D3D11_ERROR {
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

/// Direct3D / Direct3D9 errors
#[allow(non_camel_case_types)] pub struct D3DERR(()); impl D3DERR {
    /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
    pub const INVALIDCALL               : ErrorKind = ErrorKind(D3DERR_INVALIDCALL);

    /// The previous blit operation that is transferring information to or from this surface is incomplete.
    pub const WASSTILLDRAWING           : ErrorKind = ErrorKind(D3DERR_WASSTILLDRAWING);
}

/// DXGI errors
#[allow(non_camel_case_types)] pub struct DXGI_ERROR(()); impl DXGI_ERROR {
    /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
    pub const INVALID_CALL               : ErrorKind = ErrorKind(DXGI_ERROR_INVALID_CALL);

    /// The previous blit operation that is transferring information to or from this surface is incomplete.
    pub const WAS_STILL_DRAWING         : ErrorKind = ErrorKind(DXGI_ERROR_WAS_STILL_DRAWING);
}

/// General errors
#[allow(non_camel_case_types)] pub struct E(()); impl E {
    /// Attempted to create a device with the debug layer enabled and the layer is not installed.
    pub const FAIL                      : ErrorKind = ErrorKind(E_FAIL);

    /// An invalid parameter was passed to the returning function.
    pub const INVALIDARG                : ErrorKind = ErrorKind(E_INVALIDARG);

    /// Direct3D could not allocate sufficient memory to complete the call.
    pub const OUTOFMEMORY               : ErrorKind = ErrorKind(E_OUTOFMEMORY);

    /// The method call isn't implemented with the passed parameter combination.
    pub const NOTIMPL                   : ErrorKind = ErrorKind(E_NOTIMPL);
}

/// "Success" values
#[allow(non_camel_case_types)] pub struct S(()); impl S {
    /// No error occurred.
    pub const OK                        : ErrorKind = ErrorKind(S_OK);

    /// Alternate success value, indicating a successful but nonstandard completion (the precise meaning depends on context).
    pub const FALSE                     : ErrorKind = ErrorKind(S_FALSE);
}



// d3d9helper.h
const _FACD3D : u32 = 0x876;
#[allow(non_snake_case)] const fn MAKE_D3DHRESULT(code: u32) -> HRESULT { MAKE_HRESULT(1, _FACD3D, code) }
//#[allow(non_snake_case)] const fn MAKE_DDHRESULT(code: u32)  -> HRESULT { MAKE_HRESULT(1, _FACD3D, code) } // Yes, _FACD3D is the same
//#[allow(non_snake_case)] const fn MAKE_D3DSTATUS (code: u32) -> HRESULT { MAKE_HRESULT(0, _FACD3D, code) }
#[allow(non_snake_case)] const fn MAKE_HRESULT(sev: u32, fac: u32, code: u32) -> HRESULT { (sev << 31 | fac << 16 | code) as HRESULT }
