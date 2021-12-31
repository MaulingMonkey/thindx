#[allow(unused_imports)] use crate::*;

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
#[allow(non_snake_case)] const fn MAKE_DDHRESULT(code: u32)  -> D3DERR { MAKE_HRESULT(1, _FACD3D, code) } // Yes, _FACD3D is the same
#[allow(non_snake_case)] const fn MAKE_D3DSTATUS (code: u32) -> D3DERR { MAKE_HRESULT(0, _FACD3D, code) }
#[allow(non_snake_case)] const fn MAKE_HRESULT(sev: u32, fac: u32, code: u32) -> D3DERR { D3DERR((sev << 31 | fac << 16 | code) as HRESULT) }

#[allow(overflowing_literals)] impl D3DERR {
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
}



#[cfg(feature = "9ex")] impl D3D {
    /// At least one allocation that comprises the resources is on disk.
    ///
    /// Direct3D 9Ex only.
    pub const S_NOT_RESIDENT                : D3DERR = MAKE_D3DSTATUS(2165);

    /// No allocations that comprise the resources are on disk. However, at least one allocation is not in GPU-accessible memory.
    ///
    /// Direct3D 9Ex only.
    pub const S_RESIDENT_IN_SHARED_MEMORY   : D3DERR = MAKE_D3DSTATUS(2166);

    /// The desktop display mode has been changed.
    /// The application can continue rendering, but there might be color conversion/stretching.
    /// Pick a back buffer format similar to the current display mode, and call Reset to recreate the swap chains.
    /// The device will leave this state after a Reset is called.
    ///
    /// Direct3D 9Ex only.
    pub const S_PRESENT_MODE_CHANGED        : D3DERR = MAKE_D3DSTATUS(2167);

    /// The presentation area is occluded.
    /// Occlusion means that the presentation window is minimized or another device entered the fullscreen mode on the same monitor as the presentation window and the presentation window is completely on that monitor.
    /// Occlusion will not occur if the client area is covered by another Window.
    ///
    /// Occluded applications can continue rendering and all calls will succeed, but the occluded presentation window will not be updated.
    /// Preferably the application should stop rendering to the presentation window using the device and keep calling CheckDeviceState until S_OK or S_PRESENT_MODE_CHANGED returns.
    ///
    /// Direct3D 9Ex only.
    pub const S_PRESENT_OCCLUDED            : D3DERR = MAKE_D3DSTATUS(2168);
}

#[cfg(feature = "9ex")] impl D3DERR {
    /// The hardware adapter has been removed.
    /// Application must destroy the device, do enumeration of adapters and create another Direct3D device.
    /// If application continues rendering without calling Reset, the rendering calls will succeed.
    ///
    /// Applies to Direct3D 9Ex only.
    pub const DEVICEREMOVED                 : D3DERR = MAKE_D3DHRESULT(2160);

    /// The device that returned this code caused the hardware adapter to be reset by the OS.
    /// Most applications should destroy the device and quit.
    /// Applications that must continue should destroy all video memory objects (surfaces, textures, state blocks etc) and call Reset() to put the device in a default state.
    /// If the application then continues rendering in the same way, the device will return to this state.
    ///
    /// Applies to Direct3D 9Ex only.
    pub const DEVICEHUNG                    : D3DERR = MAKE_D3DHRESULT(2164);

    /// The device does not support overlay for the specified size or display mode.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const UNSUPPORTEDOVERLAY            : D3DERR = MAKE_D3DHRESULT(2171);

    /// The device does not support overlay for the specified surface format.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const UNSUPPORTEDOVERLAYFORMAT      : D3DERR = MAKE_D3DHRESULT(2172);

    /// The specified content cannot be protected.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const CANNOTPROTECTCONTENT          : D3DERR = MAKE_D3DHRESULT(2173);

    /// The specified cryptographic algorithm is not supported.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const UNSUPPORTEDCRYPTO             : D3DERR = MAKE_D3DHRESULT(2174);

    /// The present statistics have no orderly sequence.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const PRESENT_STATISTICS_DISJOINT   : D3DERR = MAKE_D3DHRESULT(2180);
}



/// [D3DXERR_*](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dxerr) constants associated with the D3DX utility library (that D3D itself might sometimes use)
impl D3DERR {
    // https://github.com/apitrace/dxsdk/blob/master/Include/d3dx9.h

    /// The index buffer cannot be modified.
    pub const CANNOTMODIFYINDEXBUFFER   : D3DERR = MAKE_DDHRESULT(2900);

    /// The mesh is invalid.
    pub const INVALIDMESH               : D3DERR = MAKE_DDHRESULT(2901);

    /// Attribute sort (D3DXMESHOPT_ATTRSORT) is not supported as an optimization technique.
    pub const CANNOTATTRSORT            : D3DERR = MAKE_DDHRESULT(2902);

    /// Skinning is not supported.
    pub const SKINNINGNOTSUPPORTED      : D3DERR = MAKE_DDHRESULT(2903);

    /// Too many influences specified.
    pub const TOOMANYINFLUENCES         : D3DERR = MAKE_DDHRESULT(2904);

    /// The data is invalid.
    pub const INVALIDDATA               : D3DERR = MAKE_DDHRESULT(2905);

    /// The mesh has no data.
    pub const LOADEDMESHASNODATA        : D3DERR = MAKE_DDHRESULT(2906);

    /// A fragment with that name already exists.
    pub const DUPLICATENAMEDFRAGMENT    : D3DERR = MAKE_DDHRESULT(2907);

    /// The last item cannot be deleted.
    pub const CANNOTREMOVELASTITEM      : D3DERR = MAKE_DDHRESULT(2908);
}



/// Undocumented / poorly documented semi-internal errors
#[allow(overflowing_literals)] impl D3DERR {
    /// The command was unparsed.
    pub const COMMAND_UNPARSED          : D3DERR = D3DERR(0x88760BB8 as _);
}



/// Errors that aren't part of the D3DERR_\* family, but might still be returned by Direct3D API calls.
#[allow(overflowing_literals)] impl D3DERR {
    /// Ran out of memory.
    pub const OUTOFMEMORY               : D3DERR = D3DERR(E_OUTOFMEMORY);

    /// No such interface supported.
    pub const NOINTERFACE               : D3DERR = D3DERR(E_NOINTERFACE);
}



/// Thin3D-specific errors
///
/// * `0xA.......`  - **S**everity and **C**ustomer bits for [HRESULT](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)s
/// * `0x.73D....`  - **T**hin **3D** error codes
/// * `0x....9001`  - Error codes
#[allow(overflowing_literals)] impl D3DERR {
    /// `0xA73D9001`    Large slice passed to D3D API that only accepts a 32-bit length
    pub const SLICE_OVERFLOW    : D3DERR = D3DERR(0xA73D9001 as _);

    /// `0xA73D9002`    Resource belonging to one [Device] was passed to a different [Device].  To avoid undefined behavior, Direct3D was not called.
    pub const DEVICE_MISMATCH   : D3DERR = D3DERR(0xA73D9002 as _);

    /// `0xA73D9003`    Large allocation was requested.  thin3d9 prevented the request to avoid arithmetic overflows inside of Direct3D / drivers which could lead to undefined behavior.
    pub const ALLOC_OVERFLOW    : D3DERR = D3DERR(0xA73D9003 as _);

    /// `0xA73D9004`    A structure contained some kind of field such as `dwSize` or `iType` that was invalid.
    pub const INVALID_STRUCT_FIELD : D3DERR = D3DERR(0xA73D9004 as _);
}



impl D3DERR {
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

            #[cfg(feature = "9ex")] D3D::S_NOT_RESIDENT                 => Some(("S_NOT_RESIDENT",                  "At least one allocation that comprises the resources is on disk.")),
            #[cfg(feature = "9ex")] D3D::S_RESIDENT_IN_SHARED_MEMORY    => Some(("S_RESIDENT_IN_SHARED_MEMORY",     "No allocations that comprise the resources are on disk. However, at least one allocation is not in GPU-accessible memory.")),
            #[cfg(feature = "9ex")] D3D::S_PRESENT_MODE_CHANGED         => Some(("S_PRESENT_MODE_CHANGED",          "The desktop display mode has been changed.")),
            #[cfg(feature = "9ex")] D3D::S_PRESENT_OCCLUDED             => Some(("S_PRESENT_OCCLUDED",              "The presentation area is occluded.")),

            #[cfg(feature = "9ex")] D3DERR::DEVICEREMOVED               => Some(("D3DERR_DEVICEREMOVED",                "The hardware adapter has been removed.")),
            #[cfg(feature = "9ex")] D3DERR::DEVICEHUNG                  => Some(("D3DERR_DEVICEHUNG",                   "The device that returned this code caused the hardware adapter to be reset by the OS.")),
            #[cfg(feature = "9ex")] D3DERR::UNSUPPORTEDOVERLAY          => Some(("D3DERR_UNSUPPORTEDOVERLAY",           "The device does not support overlay for the specified size or display mode.")),
            #[cfg(feature = "9ex")] D3DERR::UNSUPPORTEDOVERLAYFORMAT    => Some(("D3DERR_UNSUPPORTEDOVERLAYFORMAT",     "The device does not support overlay for the specified surface format.")),
            #[cfg(feature = "9ex")] D3DERR::CANNOTPROTECTCONTENT        => Some(("D3DERR_CANNOTPROTECTCONTENT",         "The specified content cannot be protected.")),
            #[cfg(feature = "9ex")] D3DERR::UNSUPPORTEDCRYPTO           => Some(("D3DERR_UNSUPPORTEDCRYPTO",            "The specified cryptographic algorithm is not supported.")),
            #[cfg(feature = "9ex")] D3DERR::PRESENT_STATISTICS_DISJOINT => Some(("D3DERR_PRESENT_STATISTICS_DISJOINT",  "The present statistics have no orderly sequence.")),

            D3DERR::CANNOTMODIFYINDEXBUFFER     => Some(("D3DXERR_CANNOTMODIFYINDEXBUFFER", "The index buffer cannot be modified.")),
            D3DERR::INVALIDMESH                 => Some(("D3DXERR_INVALIDMESH",             "The mesh is invalid.")),
            D3DERR::CANNOTATTRSORT              => Some(("D3DXERR_CANNOTATTRSORT",          "Attribute sort (D3DXMESHOPT_ATTRSORT) is not supported as an optimization technique.")),
            D3DERR::SKINNINGNOTSUPPORTED        => Some(("D3DXERR_SKINNINGNOTSUPPORTED",    "Skinning is not supported.")),
            D3DERR::TOOMANYINFLUENCES           => Some(("D3DXERR_TOOMANYINFLUENCES",       "Too many influences specified.")),
            D3DERR::INVALIDDATA                 => Some(("D3DXERR_INVALIDDATA",             "The data is invalid.")),
            D3DERR::LOADEDMESHASNODATA          => Some(("D3DXERR_LOADEDMESHASNODATA",      "The mesh has no data.")),
            D3DERR::DUPLICATENAMEDFRAGMENT      => Some(("D3DXERR_DUPLICATENAMEDFRAGMENT",  "A fragment with that name already exists.")),
            D3DERR::CANNOTREMOVELASTITEM        => Some(("D3DXERR_CANNOTREMOVELASTITEM",    "The last item cannot be deleted.")),

            D3DERR::COMMAND_UNPARSED            => Some(("D3DERR_COMMAND_UNPARSED",             "The command was unparsed.")),

            D3DERR::OUTOFMEMORY                 => Some(("E_OUTOFMEMORY",                       "Ran out of memory")),
            D3DERR::NOINTERFACE                 => Some(("E_NOINTERFACE",                       "No such interface supported")),

            D3DERR::SLICE_OVERFLOW              => Some(("THIN3DERR_SLICE_OVERFLOW",        "Large slice passed to D3D API that only accepts a 32-bit length")),
            D3DERR::DEVICE_MISMATCH             => Some(("THIN3DERR_DEVICE_MISMATCH",       "Resource belonging to one Device was passed to a different Device.  To avoid undefined behavior, Direct3D was not called.")),
            D3DERR::ALLOC_OVERFLOW              => Some(("THIN3DERR_ALLOC_OVERFLOW",        "Large allocation size passed to Device.  To avoid undefined behavior from arithmetic overflows, Direct3D was not called.")),
            D3DERR::INVALID_STRUCT_FIELD        => Some(("THIN3DERR_INVALID_STRUCT_FIELD",  "A structure contained some kind of field such as `dwSize` or `iType` that was invalid.")),

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
