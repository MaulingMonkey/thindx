#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbackbuffer-type)\]
/// D3DBACKBUFFER_TYPE
///
/// Defines constants that describe the type of back buffer.
///
/// Direct3D 9 does not support stereo view, so Direct3D does not use the [BackBufferType::Left] and [BackBufferType::Right] values of this enumerated type.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::get_back_buffer]
/// *   [SwapChain::get_back_buffer]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct BackBufferType(D3DBACKBUFFER_TYPE);

enumish! { BackBufferType => D3DBACKBUFFER_TYPE; Mono, Left, Right }

#[allow(non_upper_case_globals)] impl BackBufferType { // These are enum-like
    /// Specifies a nonstereo swap chain.
    pub const Mono  : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_MONO); // 0

    /// Specifies the left side of a stereo pair in a swap chain.
    pub const Left  : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_LEFT);

    /// Specifies the right side of a stereo pair in a swap chain.
    pub const Right : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_RIGHT);
}

#[doc(hidden)] impl BackBufferType {
    /// Specifies a nonstereo swap chain.
    pub const MONO  : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_MONO);

    /// Specifies the left side of a stereo pair in a swap chain.
    pub const LEFT  : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_LEFT);

    /// Specifies the right side of a stereo pair in a swap chain.
    pub const RIGHT : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_RIGHT);
}

impl Default for BackBufferType {
    fn default() -> Self { BackBufferType::Mono } // 0
}

//#cpp2rust D3DBACKBUFFER_TYPE          = d3d::BackBufferType
//#cpp2rust D3DBACKBUFFER_TYPE_MONO     = d3d::BackBufferType::Mono
//#cpp2rust D3DBACKBUFFER_TYPE_LEFT     = d3d::BackBufferType::Left
//#cpp2rust D3DBACKBUFFER_TYPE_RIGHT    = d3d::BackBufferType::Right
