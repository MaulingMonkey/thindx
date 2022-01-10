//! [S],
//! [D3D],
//! [E],
//! [ERROR],
//! [D3DERR],
//! [D3DXERR],
//! [D3D11_ERROR],
//! [DXGI_ERROR],
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
//!
//! *   <https://www.hresult.info/>
//! *   <https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes#system-error-codes>
//! *   <https://docs.microsoft.com/en-us/windows/win32/direct3d11/d3d11-graphics-reference-returnvalues>
//! *   <https://docs.microsoft.com/en-us/windows/win32/com/structure-of-com-error-codes>
//!
//! [HRESULT]:  https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a

#![allow(overflowing_literals)] // ErrorKind is signed for some reason
#![allow(non_snake_case)]
// TODO: Cleanup formatting etc.

use crate::*;
#[allow(unused_imports)] use crate::d3d9::*;
use winapi::shared::winerror::*;


/// `0xA7D8....` • **T**hin**DX** [ErrorKind]s
///
/// * `0xA.......`  - **S**everity and **C**ustomer bits for [HRESULT](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)s
/// * `0x.7D8....`  - **T**hin **DX** error codes
pub mod THINERR {
    use super::*;

    // General errors

    /// `0xA7D80001`    Large slice passed to a DirectX API that only accepts a 32-bit length.
    pub const SLICE_OVERFLOW    : ErrorKind = ErrorKind(0xA7D80001);

    /// `0xA7D80002`    Resource belonging to one [Device] was passed to a different [Device].  To avoid undefined behavior, DirectX was not called.
    pub const DEVICE_MISMATCH   : ErrorKind = ErrorKind(0xA7D80002);

    /// `0xA7D80003`    Large allocation size was requested.  To avoid undefined behavior from arithmetic overflows, DirectX was not called.
    pub const ALLOC_OVERFLOW    : ErrorKind = ErrorKind(0xA7D80003);

    /// `0xA7D80004`    A structure contained some kind of field such as `dwSize` or `iType` that was invalid.
    pub const INVALID_STRUCT_FIELD : ErrorKind = ErrorKind(0xA7D80004);

    /// `0xA7D80005`    This version of the DLL doesn't support this fn
    pub const MISSING_DLL_EXPORT : ErrorKind = ErrorKind(0xA7D80005);

    /// `0xA7D80006`    Slice length exceeded some kind of length limit (typically a conversion to a 32-bit length, or
    ///                 an extra cap introduced by thindx to avoid undefined behavior from allocation size overflows.)
    pub const SLICE_TOO_LARGE : ErrorKind = ErrorKind(0xA7D80006);

    /// `0xA7D80007`    String contains unexpected internal `\0`s when being passed to a function taking C-style `\0`-*terminated* strings.
    pub const STRING_CONTAINS_NULS : ErrorKind = ErrorKind(0xA7D80007);

    /// `0xA7D80008`    Bytecode is invalid (bad header, invalid checksum, wrong length, etc.)
    pub const INVALID_BYTECODE : ErrorKind = ErrorKind(0xA7D80008);
}

/// `0x887C....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d11/d3d11-graphics-reference-returnvalues)\] • Direct3D 11 [ErrorKind]s
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

/// `0x8876....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3derr)\] • Direct3D / Direct3D9 [ErrorKind]s
pub mod D3DERR {
    use super::*;

    /// The previous blit operation that is transferring information to or from this surface is incomplete.
    pub const WASSTILLDRAWING           : ErrorKind = MAKE_D3DHRESULT(540);

    /// The pixel format of the texture surface is not valid.
    pub const WRONGTEXTUREFORMAT        : ErrorKind = MAKE_D3DHRESULT(2072);

    /// The device does not support a specified texture-blending operation for color values.
    pub const UNSUPPORTEDCOLOROPERATION : ErrorKind = MAKE_D3DHRESULT(2073);

    /// The device does not support a specified texture-blending argument for color values.
    pub const UNSUPPORTEDCOLORARG       : ErrorKind = MAKE_D3DHRESULT(2074);

    /// The device does not support a specified texture-blending operation for the alpha channel.
    pub const UNSUPPORTEDALPHAOPERATION : ErrorKind = MAKE_D3DHRESULT(2075);

    /// The device does not support a specified texture-blending argument for the alpha channel.
    pub const UNSUPPORTEDALPHAARG       : ErrorKind = MAKE_D3DHRESULT(2076);

    /// The application is requesting more texture-filtering operations than the device supports.
    pub const TOOMANYOPERATIONS         : ErrorKind = MAKE_D3DHRESULT(2077);

    /// The current texture filters cannot be used together.
    pub const CONFLICTINGTEXTUREFILTER  : ErrorKind = MAKE_D3DHRESULT(2078);

    /// The device does not support the specified texture factor value. Not used; provided only to support older drivers.
    pub const UNSUPPORTEDFACTORVALUE    : ErrorKind = MAKE_D3DHRESULT(2079);

    /// The currently set render states cannot be used together.
    pub const CONFLICTINGRENDERSTATE    : ErrorKind = MAKE_D3DHRESULT(2081);

    /// The device does not support the specified texture filter.
    pub const UNSUPPORTEDTEXTUREFILTER  : ErrorKind = MAKE_D3DHRESULT(2082);

    /// The current textures cannot be used simultaneously.
    pub const CONFLICTINGTEXTUREPALETTE : ErrorKind = MAKE_D3DHRESULT(2086);

    /// Internal driver error. Applications should destroy and recreate the device when receiving this error.
    /// For hints on debugging this error, see [Driver Internal Errors (Direct3D 9)](https://docs.microsoft.com/en-us/windows/win32/direct3d9/driver-internal-errors).
    pub const DRIVERINTERNALERROR       : ErrorKind = MAKE_D3DHRESULT(2087);

    /// The requested item was not found.
    pub const NOTFOUND                  : ErrorKind = MAKE_D3DHRESULT(2150);

    /// There is more data available than the specified buffer size can hold.
    pub const MOREDATA                  : ErrorKind = MAKE_D3DHRESULT(2151);

    /// The device has been lost but cannot be reset at this time. Therefore, rendering is not possible.
    /// A Direct3D device object other than the one that returned this code caused the hardware adapter to be reset by the OS.
    /// Delete all video memory objects (surfaces, textures, state blocks) and call Reset() to return the device to a default state.
    /// If the application continues rendering without a reset, the rendering calls will succeed.
    pub const DEVICELOST                : ErrorKind = MAKE_D3DHRESULT(2152);

    /// The device has been lost but can be reset at this time.
    pub const DEVICENOTRESET            : ErrorKind = MAKE_D3DHRESULT(2153);

    /// This device does not support the queried technique.
    pub const NOTAVAILABLE              : ErrorKind = MAKE_D3DHRESULT(2154);

    /// Direct3D does not have enough display memory to perform the operation.
    /// The device is using more resources in a single scene than can fit simultaneously into video memory.
    /// [Present], [PresentEx], or [CheckDeviceState] can return this error.
    /// Recovery is similar to D3DERR_DEVICEHUNG, though the application may want to reduce its per-frame memory usage as well to avoid having the error recur.
    ///
    /// [Present]:          https://docs.microsoft.com/en-us/windows/desktop/api/d3d9/nf-d3d9-idirect3ddevice9-present
    /// [PresentEx]:        https://docs.microsoft.com/en-us/windows/desktop/api/d3d9/nf-d3d9-idirect3ddevice9ex-presentex
    /// [CheckDeviceState]: https://docs.microsoft.com/en-us/windows/desktop/api/d3d9/nf-d3d9-idirect3ddevice9ex-checkdevicestate
    pub const OUTOFVIDEOMEMORY          : ErrorKind = MAKE_D3DHRESULT(380);

    /// The requested device type is not valid.
    pub const INVALIDDEVICE             : ErrorKind = MAKE_D3DHRESULT(2155);

    /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
    pub const INVALIDCALL               : ErrorKind = MAKE_D3DHRESULT(2156);

    /// Not used.
    pub const DRIVERINVALIDCALL         : ErrorKind = MAKE_D3DHRESULT(2157);



    // Direct3D 9Ex

    /// The hardware adapter has been removed.
    /// Application must destroy the device, do enumeration of adapters and create another Direct3D device.
    /// If application continues rendering without calling Reset, the rendering calls will succeed.
    ///
    /// Applies to Direct3D 9Ex only.
    pub const DEVICEREMOVED                 : ErrorKind = MAKE_D3DHRESULT(2160);

    /// The device that returned this code caused the hardware adapter to be reset by the OS.
    /// Most applications should destroy the device and quit.
    /// Applications that must continue should destroy all video memory objects (surfaces, textures, state blocks etc) and call Reset() to put the device in a default state.
    /// If the application then continues rendering in the same way, the device will return to this state.
    ///
    /// Applies to Direct3D 9Ex only.
    pub const DEVICEHUNG                    : ErrorKind = MAKE_D3DHRESULT(2164);

    /// The device does not support overlay for the specified size or display mode.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const UNSUPPORTEDOVERLAY            : ErrorKind = MAKE_D3DHRESULT(2171);

    /// The device does not support overlay for the specified surface format.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const UNSUPPORTEDOVERLAYFORMAT      : ErrorKind = MAKE_D3DHRESULT(2172);

    /// The specified content cannot be protected.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const CANNOTPROTECTCONTENT          : ErrorKind = MAKE_D3DHRESULT(2173);

    /// The specified cryptographic algorithm is not supported.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const UNSUPPORTEDCRYPTO             : ErrorKind = MAKE_D3DHRESULT(2174);

    /// The present statistics have no orderly sequence.
    ///
    /// Direct3D 9Ex under Windows 7 only.
    pub const PRESENT_STATISTICS_DISJOINT   : ErrorKind = MAKE_D3DHRESULT(2180);



    // Undocumented / poorly documented semi-internal errors

    /// The command was unparsed.
    pub const COMMAND_UNPARSED          : ErrorKind = ErrorKind(0x88760BB8);
}

/// `0x8876....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dxerr)\] • D3DX [ErrorKind]s
///
/// Some of these can be returned by Direct3D itself
pub mod D3DXERR {
    use super::*;

    // https://github.com/apitrace/dxsdk/blob/master/Include/d3dx9.h

    /// The index buffer cannot be modified.
    pub const CANNOTMODIFYINDEXBUFFER   : ErrorKind = MAKE_DDHRESULT(2900);

    /// The mesh is invalid.
    pub const INVALIDMESH               : ErrorKind = MAKE_DDHRESULT(2901);

    /// Attribute sort (D3DXMESHOPT_ATTRSORT) is not supported as an optimization technique.
    pub const CANNOTATTRSORT            : ErrorKind = MAKE_DDHRESULT(2902);

    /// Skinning is not supported.
    pub const SKINNINGNOTSUPPORTED      : ErrorKind = MAKE_DDHRESULT(2903);

    /// Too many influences specified.
    pub const TOOMANYINFLUENCES         : ErrorKind = MAKE_DDHRESULT(2904);

    /// The data is invalid.
    pub const INVALIDDATA               : ErrorKind = MAKE_DDHRESULT(2905);

    /// The mesh has no data.
    pub const LOADEDMESHASNODATA        : ErrorKind = MAKE_DDHRESULT(2906);

    /// A fragment with that name already exists.
    pub const DUPLICATENAMEDFRAGMENT    : ErrorKind = MAKE_DDHRESULT(2907);

    /// The last item cannot be deleted.
    pub const CANNOTREMOVELASTITEM      : ErrorKind = MAKE_DDHRESULT(2908);

}

/// `0x887A....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-error)\] • DXGI [ErrorKind]s
pub mod DXGI_ERROR {
    use super::*;

    /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
    pub const INVALID_CALL              : ErrorKind = ErrorKind(DXGI_ERROR_INVALID_CALL);

    /// The previous blit operation that is transferring information to or from this surface is incomplete.
    pub const WAS_STILL_DRAWING         : ErrorKind = ErrorKind(DXGI_ERROR_WAS_STILL_DRAWING);
}

/// `0x8000....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/learnwin32/error-handling-in-com)\] • General [ErrorKind]s<br>
/// `0x8007....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/learnwin32/error-handling-in-com)\] • Win32/COM [ErrorKind]s
///
/// Errors that aren't part of the D3DERR_\* family, but might still be returned by DirectX API calls.
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

    /// No such interface supported.
    pub const NOINTERFACE               : ErrorKind = ErrorKind(E_NOINTERFACE);
}

/// `0x0000....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-)\] • Non-hresult [ErrorKind]s
pub mod ERROR {
    use super::*;

    /// No error occured
    pub const SUCCESS                   : ErrorKind = ErrorKind(ERROR_SUCCESS as _);

    /// A file was not found
    pub const FILE_NOT_FOUND            : ErrorKind = ErrorKind(ERROR_FILE_NOT_FOUND as _);

    /// Access is denied.
    pub const ACCESS_DENIED             : ErrorKind = ErrorKind(ERROR_ACCESS_DENIED as _);

    /// The system cannot find the path specified.
    pub const PATH_NOT_FOUND            : ErrorKind = ErrorKind(ERROR_PATH_NOT_FOUND as _);

    /// The file exists.
    pub const FILE_EXISTS               : ErrorKind = ErrorKind(ERROR_FILE_EXISTS as _);

    /// One or more arguments are not correct.
    pub const BAD_ARGUMENTS             : ErrorKind = ErrorKind(ERROR_BAD_ARGUMENTS as _);

    /// The device is not connected.
    pub const DEVICE_NOT_CONNECTED      : ErrorKind = ErrorKind(ERROR_DEVICE_NOT_CONNECTED as _);
}

/// `0x0000....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/learnwin32/error-handling-in-com)\] • Win32/COM success "[ErrorKind]"s<br>
/// `0x0876....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3derr)\] • Success "[ErrorKind]"s
pub mod S {
    use super::*;

    /// No error occurred.
    pub const OK                        : ErrorKind = ErrorKind(S_OK);

    /// Alternate success value, indicating a successful but nonstandard completion (the precise meaning depends on context).
    pub const FALSE                     : ErrorKind = ErrorKind(S_FALSE);

    /// At least one allocation that comprises the resources is on disk.
    ///
    /// Direct3D 9Ex only.
    pub const NOT_RESIDENT                : ErrorKind = MAKE_D3DSTATUS(2165);

    /// No allocations that comprise the resources are on disk. However, at least one allocation is not in GPU-accessible memory.
    ///
    /// Direct3D 9Ex only.
    pub const RESIDENT_IN_SHARED_MEMORY   : ErrorKind = MAKE_D3DSTATUS(2166);

    /// The desktop display mode has been changed.
    /// The application can continue rendering, but there might be color conversion/stretching.
    /// Pick a back buffer format similar to the current display mode, and call Reset to recreate the swap chains.
    /// The device will leave this state after a Reset is called.
    ///
    /// Direct3D 9Ex only.
    pub const PRESENT_MODE_CHANGED        : ErrorKind = MAKE_D3DSTATUS(2167);

    /// The presentation area is occluded.
    /// Occlusion means that the presentation window is minimized or another device entered the fullscreen mode on the same monitor as the presentation window and the presentation window is completely on that monitor.
    /// Occlusion will not occur if the client area is covered by another Window.
    ///
    /// Occluded applications can continue rendering and all calls will succeed, but the occluded presentation window will not be updated.
    /// Preferably the application should stop rendering to the presentation window using the device and keep calling CheckDeviceState until S_OK or S_PRESENT_MODE_CHANGED returns.
    ///
    /// Direct3D 9Ex only.
    pub const PRESENT_OCCLUDED            : ErrorKind = MAKE_D3DSTATUS(2168);
}

/// `0x0876....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3derr)\] • Success "[ErrorKind]"s
pub mod D3D {
    use super::*;

    /// No error occurred.
    pub const OK            : ErrorKind = ErrorKind(0);

    /// This is a success code.
    /// However, the autogeneration of mipmaps is not supported for this format.
    /// This means that resource creation will succeed but the mipmap levels will not be automatically generated.
    pub const OK_NOAUTOGEN  : ErrorKind = MAKE_D3DSTATUS(2159);
}


// d3d9helper.h
const _FACD3D : u32 = 0x876;
const fn MAKE_D3DHRESULT(code: u32) -> ErrorKind { MAKE_HRESULT(1, _FACD3D, code) }
const fn MAKE_DDHRESULT(code: u32)  -> ErrorKind { MAKE_HRESULT(1, _FACD3D, code) } // Yes, _FACD3D is the same
const fn MAKE_D3DSTATUS (code: u32) -> ErrorKind { MAKE_HRESULT(0, _FACD3D, code) }
const fn MAKE_HRESULT(sev: u32, fac: u32, code: u32) -> ErrorKind { ErrorKind((sev << 31 | fac << 16 | code) as _) }
