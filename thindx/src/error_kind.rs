use crate::*;
use winapi::shared::winerror::*;



// https://docs.microsoft.com/en-us/windows/win32/com/structure-of-com-error-codes

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/desktop/direct3d11/d3d11-graphics-reference-returnvalues)\]
/// HRESULT
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ErrorKind(pub(crate) HRESULT);

enumish! {
    ErrorKind => HRESULT; FQN;

    THINERR::MISSING_DLL_EXPORT,
    THINERR::SLICE_TOO_LARGE,
    THINERR::STRING_CONTAINS_NULS,

    D3D11_ERROR::FILE_NOT_FOUND,
    D3D11_ERROR::TOO_MANY_UNIQUE_STATE_OBJECTS,
    D3D11_ERROR::TOO_MANY_UNIQUE_VIEW_OBJECTS,
    D3D11_ERROR::DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD,

    D3DERR::INVALIDCALL,
    D3DERR::WASSTILLDRAWING,

    DXGI_ERROR::INVALID_CALL,
    DXGI_ERROR::WAS_STILL_DRAWING,

    E::FAIL,
    E::INVALIDARG,
    E::OUTOFMEMORY,
    E::NOTIMPL,

    // S
    S::FALSE,
    S::OK,
}

impl ErrorKind {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/dmerror/nf-dmerror-make_hresult)\]
    /// MAKE_HRESULT
    pub fn make_hresult(sev: u32, fac: u32, code: u32) -> Self {
        Self(((sev << 31) | (fac << 16) | (code << 0)) as _)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/dmerror/nf-dmerror-make_hresult)\]
    /// MAKE_HRESULT(1, FACILITY_WIN32, code)
    pub fn from_win32(code: u32) -> Self { Self::make_hresult(1, 7, code) }

    pub(crate) fn check(hr: HRESULT) -> Result<(), Self> {
        if !SUCCEEDED(hr) {
            Err(Self(hr))
        } else {
            Ok(())
        }
    }
}
