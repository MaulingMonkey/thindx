use winapi::shared::winerror::{HRESULT, SUCCEEDED, *};

use std::fmt::{self, Debug, Display, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3derr)\] D3DERR
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct D3DERR(pub(crate) HRESULT);

pub struct D3D;

#[allow(overflowing_literals)]
impl D3D {
    /// No error occurred.
    pub const OK            : D3DERR = D3DERR(0);

    /// This is a success code.
    /// However, the autogeneration of mipmaps is not supported for this format.
    /// This means that resource creation will succeed but the mipmap levels will not be automatically generated.
    pub const OK_NOAUTOGEN  : D3DERR = MAKE_D3DSTATUS(2159);
}

// d3d9helper.h
const _FACD3D : u32 = 0x876;
#[allow(non_snake_case)] const fn MAKE_D3DHRESULT(code: u32) -> D3DERR { MAKE_HRESULT(1, _FACD3D, code) }
#[allow(non_snake_case)] const fn MAKE_D3DSTATUS (code: u32) -> D3DERR { MAKE_HRESULT(0, _FACD3D, code) }
#[allow(non_snake_case)] const fn MAKE_HRESULT(sev: u32, fac: u32, code: u32) -> D3DERR { D3DERR((sev << 31 | fac << 16 | code) as HRESULT) }

#[allow(overflowing_literals)]
impl D3DERR {
    /// The pixel format of the texture surface is not valid.
    pub const WRONGTEXTUREFORMAT        : D3DERR = MAKE_D3DHRESULT(2072);

    /// The device does not support a specified texture-blending operation for color values.
    pub const UNSUPPORTEDCOLOROPERATION : D3DERR = MAKE_D3DHRESULT(2073);

    /// The device does not support a specified texture-blending argument for color values.
    pub const UNSUPPORTEDCOLORARG       : D3DERR = MAKE_D3DHRESULT(2074);

    /// The device does not support a specified texture-blending operation for the alpha channel.
    pub const UNSUPPORTEDALPHAOPERATION : D3DERR = MAKE_D3DHRESULT(2075);

    /// The device does not support a specified texture-blending argument for the alpha channel.
    pub const UNSUPPORTEDALPHAARG       : D3DERR = MAKE_D3DHRESULT(2076);

    /// The application is requesting more texture-filtering operations than the device supports.
    pub const TOOMANYOPERATIONS         : D3DERR = MAKE_D3DHRESULT(2077);

    /// The current texture filters cannot be used together.
    pub const CONFLICTINGTEXTUREFILTER  : D3DERR = MAKE_D3DHRESULT(2078);

    /// The device does not support the specified texture factor value. Not used; provided only to support older drivers.
    pub const UNSUPPORTEDFACTORVALUE    : D3DERR = MAKE_D3DHRESULT(2079);

    /// The currently set render states cannot be used together.
    pub const CONFLICTINGRENDERSTATE    : D3DERR = MAKE_D3DHRESULT(2081);

    /// The device does not support the specified texture filter.
    pub const UNSUPPORTEDTEXTUREFILTER  : D3DERR = MAKE_D3DHRESULT(2082);

    /// The current textures cannot be used simultaneously.
    pub const CONFLICTINGTEXTUREPALETTE : D3DERR = MAKE_D3DHRESULT(2086);

    /// Internal driver error. Applications should destroy and recreate the device when receiving this error.
    /// For hints on debugging this error, see [Driver Internal Errors (Direct3D 9)](https://docs.microsoft.com/en-us/windows/win32/direct3d9/driver-internal-errors).
    pub const DRIVERINTERNALERROR       : D3DERR = MAKE_D3DHRESULT(2087);



    /// The requested item was not found.
    pub const NOTFOUND                  : D3DERR = MAKE_D3DHRESULT(2150);

    /// There is more data available than the specified buffer size can hold.
    pub const MOREDATA                  : D3DERR = MAKE_D3DHRESULT(2151);

    /// The device has been lost but cannot be reset at this time. Therefore, rendering is not possible.
    /// A Direct3D device object other than the one that returned this code caused the hardware adapter to be reset by the OS.
    /// Delete all video memory objects (surfaces, textures, state blocks) and call Reset() to return the device to a default state.
    /// If the application continues rendering without a reset, the rendering calls will succeed.
    pub const DEVICELOST                : D3DERR = MAKE_D3DHRESULT(2152);

    /// The device has been lost but can be reset at this time.
    pub const DEVICENOTRESET            : D3DERR = MAKE_D3DHRESULT(2153);

    /// This device does not support the queried technique.
    pub const NOTAVAILABLE              : D3DERR = MAKE_D3DHRESULT(2154);

    /// Direct3D does not have enough display memory to perform the operation.
    /// The device is using more resources in a single scene than can fit simultaneously into video memory.
    /// [Present], [PresentEx], or [CheckDeviceState] can return this error.
    /// Recovery is similar to D3DERR_DEVICEHUNG, though the application may want to reduce its per-frame memory usage as well to avoid having the error recur.
    ///
    /// [Present]:          https://docs.microsoft.com/en-us/windows/desktop/api/d3d9/nf-d3d9-idirect3ddevice9-present
    /// [PresentEx]:        https://docs.microsoft.com/en-us/windows/desktop/api/d3d9/nf-d3d9-idirect3ddevice9ex-presentex
    /// [CheckDeviceState]: https://docs.microsoft.com/en-us/windows/desktop/api/d3d9/nf-d3d9-idirect3ddevice9ex-checkdevicestate
    pub const OUTOFVIDEOMEMORY          : D3DERR = MAKE_D3DHRESULT(380);

    /// The requested device type is not valid.
    pub const INVALIDDEVICE             : D3DERR = MAKE_D3DHRESULT(2155);

    /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
    pub const INVALIDCALL               : D3DERR = MAKE_D3DHRESULT(2156);

    /// Not used.
    pub const DRIVERINVALIDCALL         : D3DERR = MAKE_D3DHRESULT(2157);

    /// The previous blit operation that is transferring information to or from this surface is incomplete.
    pub const WASSTILLDRAWING           : D3DERR = MAKE_D3DHRESULT(540);



    /// Ran out of memory.
    pub const OUTOFMEMORY               : D3DERR = D3DERR(E_OUTOFMEMORY);
}

impl D3DERR {
    fn id_desc(self) -> Option<(&'static str, &'static str)> {
        match self {
            D3D::OK                             => Some(("D3D_OK",                              "No error occured.")),
            D3D::OK_NOAUTOGEN                   => Some(("D3DOK_NOAUTOGEN",                     "This is a success code. However, the autogeneration of mipmaps is not supported for this format. This means that resource creation will succeed but the mipmap levels will not be automatically generated.")),

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
            D3DERR::DEVICELOST                  => Some(("D3DERR_DEVICELOST",                   "The device has been lost but cannot be reset at this time. Therefore, rendering is not possible. A Direct3D device object other than the one that returned this code caused the hardware adapter to be reset by the OS. Delete all video memory objects (surfaces, textures, state blocks) and call Reset() to return the device to a default state. If the application continues rendering without a reset, the rendering calls will succeed.")),
            D3DERR::DEVICENOTRESET              => Some(("D3DERR_DEVICENOTRESET",               "The device has been lost but can be reset at this time.")),
            D3DERR::NOTAVAILABLE                => Some(("D3DERR_NOTAVAILABLE",                 "This device does not support the queried technique.")),
            D3DERR::OUTOFVIDEOMEMORY            => Some(("D3DERR_OUTOFVIDEOMEMORY",             "Direct3D does not have enough display memory to perform the operation. The device is using more resources in a single scene than can fit simultaneously into video memory. Present, PresentEx, or CheckDeviceState can return this error. Recovery is similar to D3DERR_DEVICEHUNG, though the application may want to reduce its per-frame memory usage as well to avoid having the error recur.")),
            D3DERR::INVALIDDEVICE               => Some(("D3DERR_INVALIDDEVICE",                "The requested device type is not valid.")),
            D3DERR::INVALIDCALL                 => Some(("D3DERR_INVALIDCALL",                  "The method call is invalid. For example, a method's parameter may not be a valid pointer.")),
            D3DERR::DRIVERINVALIDCALL           => Some(("D3DERR_DRIVERINVALIDCALL",            "Not used.")),
            D3DERR::WASSTILLDRAWING             => Some(("D3DERR_WASSTILLDRAWING",              "The previous blit operation that is transferring information to or from this surface is incomplete.")),

            D3DERR::OUTOFMEMORY                 => Some(("E_OUTOFMEMORY",                       "Ran out of memory")),

            _                                   => None,
        }
    }
}

impl Display for D3DERR {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some((id, desc)) = self.id_desc() {
            write!(f, "{} (0x{:08x}) {}", id, self.0 as u32, desc)
        } else {
            write!(f, "D3DERR_??? (0x{:08x})", self.0 as u32)
        }
    }
}

impl Debug for D3DERR {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some((id, _desc)) = self.id_desc() {
            write!(f, "{}", id)
        } else {
            write!(f, "D3DERR(0x{:08x})", self.0 as u32)
        }
    }
}

impl std::error::Error for D3DERR {}

impl PartialEq<HRESULT> for D3DERR { fn eq(&self, other: &HRESULT)   -> bool { self.0 == *other } }
impl PartialEq<D3DERR> for HRESULT { fn eq(&self, other: &D3DERR)    -> bool { other.0 == *self } }

impl PartialEq<Option<D3DERR>> for D3DERR { fn eq(&self, other: &Option<D3DERR>) -> bool { Some(self) == other.as_ref() } }
impl PartialEq<D3DERR> for Option<D3DERR> { fn eq(&self, other: &D3DERR)         -> bool { Some(other) == self.as_ref() } }

impl PartialEq<Option<MethodError>> for D3DERR { fn eq(&self, other: &Option<MethodError>) -> bool { Some(*self) == other.as_ref().map(|e| e.d3derr()) } }
impl PartialEq<D3DERR> for Option<MethodError> { fn eq(&self, other: &D3DERR)              -> bool { Some(*other) == self.as_ref().map(|e| e.d3derr()) } }

impl<O> PartialEq<Result<O, D3DERR>> for D3DERR { fn eq(&self, other: &Result<O, D3DERR>) -> bool { Some(self) == other.as_ref().err() } }
impl<O> PartialEq<D3DERR> for Result<O, D3DERR> { fn eq(&self, other: &D3DERR)            -> bool { Some(other) == self.as_ref().err() } }

impl<O> PartialEq<Result<O, MethodError>> for D3DERR { fn eq(&self, other: &Result<O, MethodError>) -> bool { Some(*self) == other.as_ref().err().map(|e| e.d3derr()) } }
impl<O> PartialEq<D3DERR> for Result<O, MethodError> { fn eq(&self, other: &D3DERR)                 -> bool { Some(*other) == self.as_ref().err().map(|e| e.d3derr()) } }



/// An error about some specific method returning an [HRESULT](https://www.hresult.info/)
#[derive(Clone)]
pub struct MethodError(pub(crate) &'static str, pub(crate) D3DERR);

impl MethodError {
    /// Returns an `Err(MethodError(...))` if `!SUCCEEDED(hr)`
    pub fn check(method: &'static str, hr: HRESULT) -> Result<(), Self> {
        if SUCCEEDED(hr) {
            Ok(())
        } else {
            Err(MethodError(method, D3DERR(hr)))
        }
    }

    /// Returns the [D3DERR] of the error
    pub fn d3derr(&self) -> D3DERR { self.1 }

    /// Returns the [HRESULT] of the error
    pub fn hresult(&self) -> HRESULT { self.1.0 }

    /// Returns a link in the format of e.g. "<https://www.hresult.info/Search?q=0x80000005>"
    pub fn hresult_info_search_link(&self) -> String { format!("https://www.hresult.info/Search?q=0x{:08x}", self.1.0 as u32) }
}

impl Debug   for MethodError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "MethodError({:?}, {:?})", self.0, self.1) } }
impl Display for MethodError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} failed with HRESULT == {}", self.0, self.1) } }

impl std::error::Error for MethodError {}
