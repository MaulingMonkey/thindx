use crate::*;

use bytemuck::*;

use winapi::shared::winerror::*;

use std::fmt::{self, Debug, Display, Formatter};



// https://docs.microsoft.com/en-us/windows/win32/com/structure-of-com-error-codes
// https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3derr

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/desktop/direct3d11/d3d11-graphics-reference-returnvalues)\]
/// HRESULT
///
/// See [thindx::errors](crate::errors) for a list of constants
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ErrorKind(pub(crate) HRESULT);

enumish! { ErrorKind => HRESULT }

impl ErrorKind {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/dmerror/nf-dmerror-make_hresult)\]
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

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/dmerror/nf-dmerror-make_hresult)\]
    /// MAKE_HRESULT(1, FACILITY_WIN32, code)
    ///
    /// ### Arguments
    /// *   `code`  - the `ERROR_*` code/value (e.g. `ERROR_FILE_NOT_FOUND`)
    pub const fn from_win32(code: u32) -> Self { Self::make_hresult(1, 7, code) }

    fn id_desc(self) -> Option<(&'static str, &'static str)> {
        match self {
            D3D::OK                             => Some(("D3D_OK",                              "No error occured.")),
            D3D::OK_NOAUTOGEN                   => Some(("D3DOK_NOAUTOGEN",                     "The autogeneration of mipmaps is not supported for this format.")),

            D3DERR::WRONGTEXTUREFORMAT          => Some(("D3DERR_WRONGTEXTUREFORMAT",           "The pixel format of the texture surface is not valid.")),
            D3DERR::UNSUPPORTEDCOLOROPERATION   => Some(("D3DERR_UNSUPPORTEDCOLOROPERATION",    "The device does not support a specified texture-blending operation for color values.")),
            D3DERR::UNSUPPORTEDCOLORARG         => Some(("D3DERR_UNSUPPORTEDCOLORARG",          "The device does not support a specified texture-blending argument for color values.")),
            D3DERR::UNSUPPORTEDALPHAOPERATION   => Some(("D3DERR_UNSUPPORTEDALPHAOPERATION",    "The device does not support a specified texture-blending operation for the alpha channel.")),
            D3DERR::UNSUPPORTEDALPHAARG         => Some(("D3DERR_UNSUPPORTEDALPHAARG",          "The device does not support a specified texture-blending argument for the alpha channel.")),
            D3DERR::TOOMANYOPERATIONS           => Some(("D3DERR_TOOMANYOPERATIONS",            "The application is requesting more texture-filtering operations than the device supports.")),
            D3DERR::CONFLICTINGTEXTUREFILTER    => Some(("D3DERR_CONFLICTINGTEXTUREFILTER",     "The current texture filters cannot be used together.")),
            D3DERR::UNSUPPORTEDFACTORVALUE      => Some(("D3DERR_UNSUPPORTEDFACTORVALUE",       "The device does not support the specified texture factor value. Not used; provided only to support older drivers.")),
            D3DERR::CONFLICTINGRENDERSTATE      => Some(("D3DERR_CONFLICTINGRENDERSTATE",       "The currently set render states cannot be used together.")),
            D3DERR::UNSUPPORTEDTEXTUREFILTER    => Some(("D3DERR_UNSUPPORTEDTEXTUREFILTER",     "The device does not support the specified texture filter.")),
            D3DERR::CONFLICTINGTEXTUREPALETTE   => Some(("D3DERR_CONFLICTINGTEXTUREPALETTE",    "The current textures cannot be used simultaneously.")),
            D3DERR::DRIVERINTERNALERROR         => Some(("D3DERR_DRIVERINTERNALERROR",          "Internal driver error. Applications should destroy and recreate the device when receiving this error.")),

            D3DERR::NOTFOUND                    => Some(("D3DERR_NOTFOUND",                     "The requested item was not found.")),
            D3DERR::MOREDATA                    => Some(("D3DERR_MOREDATA",                     "There is more data available than the specified buffer size can hold.")),
            D3DERR::DEVICELOST                  => Some(("D3DERR_DEVICELOST",                   "The device has been lost but cannot be reset at this time.")),
            D3DERR::DEVICENOTRESET              => Some(("D3DERR_DEVICENOTRESET",               "The device has been lost but can be reset at this time.")),
            D3DERR::NOTAVAILABLE                => Some(("D3DERR_NOTAVAILABLE",                 "This device does not support the queried technique.")),
            D3DERR::OUTOFVIDEOMEMORY            => Some(("D3DERR_OUTOFVIDEOMEMORY",             "Direct3D does not have enough display memory to perform the operation. The device is using more resources in a single scene than can fit simultaneously into video memory.")),
            D3DERR::INVALIDDEVICE               => Some(("D3DERR_INVALIDDEVICE",                "The requested device type is not valid.")),
            D3DERR::INVALIDCALL                 => Some(("D3DERR_INVALIDCALL",                  "The method call is invalid. For example, a method's parameter may not be a valid pointer.")),
            D3DERR::DRIVERINVALIDCALL           => Some(("D3DERR_DRIVERINVALIDCALL",            "Not used.")),
            D3DERR::WASSTILLDRAWING             => Some(("D3DERR_WASSTILLDRAWING",              "The previous blit operation that is transferring information to or from this surface is incomplete.")),

            #[allow(unreachable_patterns)] // D3D_OK
            S::OK                               => Some(("S_OK",                            "No error occurred.")),
            S::FALSE                            => Some(("S_FALSE",                         "Alternate success value, indicating a successful but nonstandard completion.")),
            S::NOT_RESIDENT                     => Some(("S_NOT_RESIDENT",                  "At least one allocation that comprises the resources is on disk.")),
            S::RESIDENT_IN_SHARED_MEMORY        => Some(("S_RESIDENT_IN_SHARED_MEMORY",     "No allocations that comprise the resources are on disk. However, at least one allocation is not in GPU-accessible memory.")),
            S::PRESENT_MODE_CHANGED             => Some(("S_PRESENT_MODE_CHANGED",          "The desktop display mode has been changed.")),
            S::PRESENT_OCCLUDED                 => Some(("S_PRESENT_OCCLUDED",              "The presentation area is occluded.")),

            D3DERR::DEVICEREMOVED               => Some(("D3DERR_DEVICEREMOVED",                "The hardware adapter has been removed.")),
            D3DERR::DEVICEHUNG                  => Some(("D3DERR_DEVICEHUNG",                   "The device that returned this code caused the hardware adapter to be reset by the OS.")),
            D3DERR::UNSUPPORTEDOVERLAY          => Some(("D3DERR_UNSUPPORTEDOVERLAY",           "The device does not support overlay for the specified size or display mode.")),
            D3DERR::UNSUPPORTEDOVERLAYFORMAT    => Some(("D3DERR_UNSUPPORTEDOVERLAYFORMAT",     "The device does not support overlay for the specified surface format.")),
            D3DERR::CANNOTPROTECTCONTENT        => Some(("D3DERR_CANNOTPROTECTCONTENT",         "The specified content cannot be protected.")),
            D3DERR::UNSUPPORTEDCRYPTO           => Some(("D3DERR_UNSUPPORTEDCRYPTO",            "The specified cryptographic algorithm is not supported.")),
            D3DERR::PRESENT_STATISTICS_DISJOINT => Some(("D3DERR_PRESENT_STATISTICS_DISJOINT",  "The present statistics have no orderly sequence.")),

            D3DXERR::CANNOTMODIFYINDEXBUFFER    => Some(("D3DXERR_CANNOTMODIFYINDEXBUFFER", "The index buffer cannot be modified.")),
            D3DXERR::INVALIDMESH                => Some(("D3DXERR_INVALIDMESH",             "The mesh is invalid.")),
            D3DXERR::CANNOTATTRSORT             => Some(("D3DXERR_CANNOTATTRSORT",          "Attribute sort (D3DXMESHOPT_ATTRSORT) is not supported as an optimization technique.")),
            D3DXERR::SKINNINGNOTSUPPORTED       => Some(("D3DXERR_SKINNINGNOTSUPPORTED",    "Skinning is not supported.")),
            D3DXERR::TOOMANYINFLUENCES          => Some(("D3DXERR_TOOMANYINFLUENCES",       "Too many influences specified.")),
            D3DXERR::INVALIDDATA                => Some(("D3DXERR_INVALIDDATA",             "The data is invalid.")),
            D3DXERR::LOADEDMESHASNODATA         => Some(("D3DXERR_LOADEDMESHASNODATA",      "The mesh has no data.")),
            D3DXERR::DUPLICATENAMEDFRAGMENT     => Some(("D3DXERR_DUPLICATENAMEDFRAGMENT",  "A fragment with that name already exists.")),
            D3DXERR::CANNOTREMOVELASTITEM       => Some(("D3DXERR_CANNOTREMOVELASTITEM",    "The last item cannot be deleted.")),

            DXGI_ERROR::INVALID_CALL            => Some(("DXGI_ERROR_INVALID_CALL",         "The method call is invalid. For example, a method's parameter may not be a valid pointer.")),
            DXGI_ERROR::WAS_STILL_DRAWING       => Some(("DXGI_ERROR_WAS_STILL_DRAWING",    "The previous blit operation that is transferring information to or from this surface is incomplete.")),

            D3DERR::COMMAND_UNPARSED            => Some(("D3DERR_COMMAND_UNPARSED",             "The command was unparsed.")),

            E::FAIL                             => Some(("E_FAIL",                              "A generic error, such as attempting to create a device with the debug layer enabled and the layer is not installed.")),
            E::INVALIDARG                       => Some(("E_INVALIDARG",                        "An invalid parameter was passed to the returning function.")),
            E::OUTOFMEMORY                      => Some(("E_OUTOFMEMORY",                       "Ran out of memory")),
            E::NOTIMPL                          => Some(("E_NOTIMPL",                           "The method call isn't implemented with the passed parameter combination.")),
            E::ACCESSDENIED                     => Some(("E_ACCESSDENIED",                      "Access is denied.")),
            E::NOINTERFACE                      => Some(("E_NOINTERFACE",                       "No such interface supported")),

            ERROR::FILE_NOT_FOUND               => Some(("ERROR_FILE_NOT_FOUND",                "A file was not found")),
            ERROR::ACCESS_DENIED                => Some(("ERROR_ACCESS_DENIED",                 "Access is denied.")),
            ERROR::PATH_NOT_FOUND               => Some(("ERROR_PATH_NOT_FOUND",                "The system cannot find the path specified.")),
            ERROR::FILE_EXISTS                  => Some(("ERROR_FILE_EXISTS",                   "The file exists.")),
            ERROR::BAD_ARGUMENTS                => Some(("ERROR_BAD_ARGUMENTS",                 "One or more arguments are not correct.")),
            ERROR::DEVICE_NOT_CONNECTED         => Some(("ERROR_DEVICE_NOT_CONNECTED",          "The device is not connected.")),
            ERROR::INVALID_WINDOW_HANDLE        => Some(("ERROR_INVALID_WINDOW_HANDLE",         "Invalid window handle.")),

            THINERR::NONSPECIFIC                => Some(("THINERR_NONSPECIFIC",             "A nonspecific error of some sort occured.")),
            THINERR::DEVICE_MISMATCH            => Some(("THINERR_DEVICE_MISMATCH",         "Resource belonging to one Device was passed to a different Device.  To avoid undefined behavior, DirectX was not called.")),
            THINERR::ALLOC_OVERFLOW             => Some(("THINERR_ALLOC_OVERFLOW",          "Large allocation size was requested.  To avoid undefined behavior from arithmetic overflows, DirectX was not called.")),
            THINERR::INVALID_STRUCT_FIELD       => Some(("THINERR_INVALID_STRUCT_FIELD",    "A structure contained some kind of field such as `dwSize` or `iType` that was invalid.")),
            THINERR::MISSING_DLL_EXPORT         => Some(("THINERR_MISSING_DLL_EXPORT",      "This version of the DLL doesn't support this fn.")),
            THINERR::SLICE_TOO_LARGE            => Some(("THINERR_SLICE_TOO_LARGE",         "Slice length exceeded some kind of length limit.")),
            THINERR::STRING_CONTAINS_NULS       => Some(("THINERR_STRING_CONTAINS_NULS",    "String contains unexpected internal `\\0`s when being passed to a function taking C-style `\\0`-*terminated* strings.")),
            THINERR::INVALID_BYTECODE           => Some(("THINERR_INVALID_BYTECODE",        "Bytecode is invalid (bad header, invalid checksum, wrong length, etc.)")),

            D3D11_ERROR::FILE_NOT_FOUND                                 => Some(("D3D11_ERROR_FILE_NOT_FOUND",                                  "The file was not found.")),
            D3D11_ERROR::TOO_MANY_UNIQUE_STATE_OBJECTS                  => Some(("D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS",                   "There are too many unique instances of a particular type of state object.")),
            D3D11_ERROR::TOO_MANY_UNIQUE_VIEW_OBJECTS                   => Some(("D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS",                    "There are too many unique instances of a particular type of view object.")),
            D3D11_ERROR::DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD   => Some(("D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD",    "The first call to ID3D11DeviceContext::Map after either ID3D11Device::CreateDeferredContext or ID3D11DeviceContext::FinishCommandList per Resource was not D3D11_MAP_WRITE_DISCARD.")),


            _                                   => None,
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some((id, desc)) = self.id_desc() {
            write!(f, "{} (0x{:08x}) {}", id, self.0 as u32, desc)
        } else {
            write!(f, "D3DERR_??? (0x{:08x})", self.0 as u32)
        }
    }
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some((id, _desc)) = self.id_desc() {
            write!(f, "{}", id)
        } else {
            write!(f, "ErrorKind(0x{:08x})", self.0 as u32)
        }
    }
}

impl std::error::Error for ErrorKind {}

impl From<std::ffi::NulError> for ErrorKind {
    fn from(_: std::ffi::NulError) -> ErrorKind { THINERR::STRING_CONTAINS_NULS }
}

impl From<abistr::InteriorNulError> for ErrorKind {
    fn from(_: abistr::InteriorNulError) -> ErrorKind { THINERR::STRING_CONTAINS_NULS }
}

impl PartialEq<HRESULT> for ErrorKind { fn eq(&self, other: &HRESULT)   -> bool { self.0 == *other } }
impl PartialEq<ErrorKind> for HRESULT { fn eq(&self, other: &ErrorKind)    -> bool { other.0 == *self } }

impl PartialEq<Option<ErrorKind>> for ErrorKind { fn eq(&self, other: &Option<ErrorKind>) -> bool { Some(self) == other.as_ref() } }
impl PartialEq<ErrorKind> for Option<ErrorKind> { fn eq(&self, other: &ErrorKind)         -> bool { Some(other) == self.as_ref() } }

impl<O> PartialEq<Result<O, ErrorKind>> for ErrorKind { fn eq(&self, other: &Result<O, ErrorKind>) -> bool { Some(self) == other.as_ref().err() } }
impl<O> PartialEq<ErrorKind> for Result<O, ErrorKind> { fn eq(&self, other: &ErrorKind)            -> bool { Some(other) == self.as_ref().err() } }

const _FACD3D : u32 = 0x876; // d3d9helper.h

//#cpp2rust HRESULT         = ErrorKind
//#cpp2rust MAKE_HRESULT    = ErrorKind::make_hresult

//#cpp2rust MAKE_D3DSTATUS  = ErrorKind::make_d3dstatus
//#cpp2rust MAKE_D3DHRESULT = ErrorKind::make_d3dhresult
//#cpp2rust MAKE_DDHRESULT  = ErrorKind::make_ddhresult
