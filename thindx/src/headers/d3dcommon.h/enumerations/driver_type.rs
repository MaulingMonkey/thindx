#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_driver_type)\]
/// D3D_DRIVER_TYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct DriverType(D3D_DRIVER_TYPE);

enumish! { DriverType => D3D_DRIVER_TYPE; default: Unknown == 0; Unknown, Hardware, Reference, Null, Software, WARP }

#[allow(non_upper_case_globals)] impl DriverType { // These are enum-like
    /// The device type is unknown.
    ///
    /// You must use this when calling
    /// [D3D11CreateDevice](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-d3d11createdevice)
    /// (and friends) on a specific adapter, since the adapter dictates what driver type is used.
    pub const Unknown   : DriverType = DriverType(D3D_DRIVER_TYPE_UNKNOWN); // 0

    /// Use a hardware driver (e.g. your graphics card.)
    ///
    /// This is the most common
    pub const Hardware  : DriverType = DriverType(D3D_DRIVER_TYPE_HARDWARE);

    /// Use the slow software "reference" driver.
    ///
    /// This is intended for development/debugging only, and requires the Windows 8 SDK (or higher).
    pub const Reference : DriverType = DriverType(D3D_DRIVER_TYPE_REFERENCE);

    /// Use the non-rendering "null" reference driver.
    ///
    /// This is intended for development/debugging only, and requires the DirectX SDK.
    pub const Null      : DriverType = DriverType(D3D_DRIVER_TYPE_NULL);

    /// Use a custom software driver.
    ///
    /// Given it's slow performance, you probably don't want this.
    pub const Software  : DriverType = DriverType(D3D_DRIVER_TYPE_SOFTWARE);

    /// Use [Windows Advanced Rasterization Platform (WARP)](https://learn.microsoft.com/en-us/windows/win32/direct3darticles/directx-warp),
    /// a high performance software rasterizer in Windows 8 (and perhaps Windows 7 SP2+?).
    ///
    /// While slower than a dedicated GPU, it *is* optimized for performance (JIT compiling shaders taking advantage of SSE etc.)
    /// and retail use (doesn't require any developer-focused SDKs to be installed, just the regular Direct3D11 runtime.)
    pub const WARP      : DriverType = DriverType(D3D_DRIVER_TYPE_WARP);
}

//#cpp2rust D3D_DRIVER_TYPE             = d3d::DriverType

//#cpp2rust D3D_DRIVER_TYPE_UNKNOWN     = d3d::DriverType::Unknown
//#cpp2rust D3D_DRIVER_TYPE_HARDWARE    = d3d::DriverType::Hardware
//#cpp2rust D3D_DRIVER_TYPE_REFERENCE   = d3d::DriverType::Reference
//#cpp2rust D3D_DRIVER_TYPE_NULL        = d3d::DriverType::Null
//#cpp2rust D3D_DRIVER_TYPE_SOFTWARE    = d3d::DriverType::Software
//#cpp2rust D3D_DRIVER_TYPE_WARP        = d3d::DriverType::WARP
