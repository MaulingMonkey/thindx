use winapi::shared::winerror::*;

const D3DERR_INVALIDCALL        : HRESULT = MAKE_D3DHRESULT(2156);
const D3DERR_WASSTILLDRAWING    : HRESULT = MAKE_D3DHRESULT(540);



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/desktop/direct3d11/d3d11-graphics-reference-returnvalues)\] HRESULT
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ErrorKind(pub(crate) HRESULT);

#[doc(hidden)] pub use ErrorKind as D3D11_ERROR;
#[doc(hidden)] pub use ErrorKind as D3DERR;
#[doc(hidden)] pub use ErrorKind as DXGI_ERROR;
#[doc(hidden)] pub use ErrorKind as E;
#[doc(hidden)] pub use ErrorKind as S;

enumish! {
    ErrorKind => HRESULT;

    FILE_NOT_FOUND,
    TOO_MANY_UNIQUE_STATE_OBJECTS,
    TOO_MANY_UNIQUE_VIEW_OBJECTS,
    DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD,
    INVALIDCALL,
    WASSTILLDRAWING,
    INVALID_CALL,
    WAS_STILL_DRAWING,
    FAIL,
    INVALIDARG,
    OUTOFMEMORY,
    NOTIMPL,
    FALSE,
    OK,
}

// TODO: CamelCase errors?

// https://docs.microsoft.com/en-us/windows/win32/direct3d11/d3d11-graphics-reference-returnvalues

impl ErrorKind {
    pub(crate) fn check(hr: HRESULT) -> Result<(), Self> {
        if !SUCCEEDED(hr) {
            Err(Self(hr))
        } else {
            Ok(())
        }
    }
}

impl D3D11_ERROR {
    /// The file was not found.
    pub const FILE_NOT_FOUND                                : D3D11_ERROR = D3D11_ERROR(D3D11_ERROR_FILE_NOT_FOUND);

    /// There are too many unique instances of a particular type of state object.
    pub const TOO_MANY_UNIQUE_STATE_OBJECTS                 : D3D11_ERROR = D3D11_ERROR(D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS);

    /// There are too many unique instances of a particular type of view object.
    pub const TOO_MANY_UNIQUE_VIEW_OBJECTS                  : D3D11_ERROR = D3D11_ERROR(D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS);

    /// The first call to [ID3D11DeviceContext::Map] after either [ID3D11Device::CreateDeferredContext] or [ID3D11DeviceContext::FinishCommandList] per Resource was not D3D11_MAP_WRITE_DISCARD.
    ///
    /// [ID3D11DeviceContext::Map]:                 https://docs.microsoft.com/en-us/windows/desktop/api/D3D11/nf-d3d11-id3d11devicecontext-map
    /// [ID3D11Device::CreateDeferredContext]:      https://docs.microsoft.com/en-us/windows/desktop/api/D3D11/nf-d3d11-id3d11device-createdeferredcontext
    /// [ID3D11DeviceContext::FinishCommandList]:   https://docs.microsoft.com/en-us/windows/desktop/api/D3D11/nf-d3d11-id3d11devicecontext-finishcommandlist
    pub const DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD  : D3D11_ERROR = D3D11_ERROR(D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD);
}

impl D3DERR {
    /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
    pub const INVALIDCALL               : D3DERR = D3DERR(D3DERR_INVALIDCALL);

    /// The previous blit operation that is transferring information to or from this surface is incomplete.
    pub const WASSTILLDRAWING           : D3DERR = D3DERR(D3DERR_WASSTILLDRAWING);
}

impl DXGI_ERROR {
    /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
    pub const INVALID_CALL               : DXGI_ERROR = DXGI_ERROR(DXGI_ERROR_INVALID_CALL);

    /// The previous blit operation that is transferring information to or from this surface is incomplete.
    pub const WAS_STILL_DRAWING         : DXGI_ERROR = DXGI_ERROR(DXGI_ERROR_WAS_STILL_DRAWING);
}

impl E {
    /// Attempted to create a device with the debug layer enabled and the layer is not installed.
    pub const FAIL                      : E = E(E_FAIL);

    /// An invalid parameter was passed to the returning function.
    pub const INVALIDARG                : E = E(E_INVALIDARG);

    /// Direct3D could not allocate sufficient memory to complete the call.
    pub const OUTOFMEMORY               : E = E(E_OUTOFMEMORY);

    /// The method call isn't implemented with the passed parameter combination.
    pub const NOTIMPL                   : E = E(E_NOTIMPL);
}

impl S {
    /// Alternate success value, indicating a successful but nonstandard completion (the precise meaning depends on context).
    pub const FALSE                     : S = S(S_FALSE);

    /// No error occurred.
    pub const OK                        : S = S(S_OK);
}



// d3d9helper.h
const _FACD3D : u32 = 0x876;
#[allow(non_snake_case)] const fn MAKE_D3DHRESULT(code: u32) -> HRESULT { MAKE_HRESULT(1, _FACD3D, code) }
//#[allow(non_snake_case)] const fn MAKE_DDHRESULT(code: u32)  -> HRESULT { MAKE_HRESULT(1, _FACD3D, code) } // Yes, _FACD3D is the same
//#[allow(non_snake_case)] const fn MAKE_D3DSTATUS (code: u32) -> HRESULT { MAKE_HRESULT(0, _FACD3D, code) }
#[allow(non_snake_case)] const fn MAKE_HRESULT(sev: u32, fac: u32, code: u32) -> HRESULT { (sev << 31 | fac << 16 | code) as HRESULT }
