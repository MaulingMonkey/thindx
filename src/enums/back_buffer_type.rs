#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbackbuffer-type)\]
/// D3DBACKBUFFER_TYPE
///
/// Defines constants that describe the type of back buffer.
///
/// Direct3D 9 does not support stereo view, so Direct3D does not use the D3DBACKBUFFER_TYPE_LEFT and D3DBACKBUFFER_TYPE_RIGHT values of this enumerated type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct BackBufferType(D3DBACKBUFFER_TYPE);

enumish! { BackBufferType => D3DBACKBUFFER_TYPE; Mono, Left, Right }

#[allow(non_upper_case_globals)] // These are enum-like
impl BackBufferType {
    /// Specifies a nonstereo swap chain.
    pub const Mono  : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_MONO);

    /// Specifies the left side of a stereo pair in a swap chain.
    pub const Left  : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_LEFT);

    /// Specifies the right side of a stereo pair in a swap chain.
    pub const Right : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_RIGHT);
}

impl Default for BackBufferType {
    fn default() -> Self { BackBufferType::Mono }
}
