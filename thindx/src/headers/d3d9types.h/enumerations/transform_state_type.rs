#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtransformstatetype)\]
/// D3DTRANSFORMSTATETYPE
///
/// Defines constants that describe transformation state values.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TransformStateType(D3DTRANSFORMSTATETYPE);
pub use TransformStateType as TS;

enumish! { TS => D3DTRANSFORMSTATETYPE; View, Projection, Texture0, Texture1, Texture2, Texture3, Texture4, Texture5, Texture6, Texture7 }

#[allow(non_upper_case_globals)] impl TransformStateType { // These are enum-like
    pub const View          : TransformStateType = TransformStateType(D3DTS_VIEW); // 2
    pub const Projection    : TransformStateType = TransformStateType(D3DTS_PROJECTION);
    pub const Texture0      : TransformStateType = TransformStateType(D3DTS_TEXTURE0);
    pub const Texture1      : TransformStateType = TransformStateType(D3DTS_TEXTURE1);
    pub const Texture2      : TransformStateType = TransformStateType(D3DTS_TEXTURE2);
    pub const Texture3      : TransformStateType = TransformStateType(D3DTS_TEXTURE3);
    pub const Texture4      : TransformStateType = TransformStateType(D3DTS_TEXTURE4);
    pub const Texture5      : TransformStateType = TransformStateType(D3DTS_TEXTURE5);
    pub const Texture6      : TransformStateType = TransformStateType(D3DTS_TEXTURE6);
    pub const Texture7      : TransformStateType = TransformStateType(D3DTS_TEXTURE7);

    // https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dts-worldn
    pub const World         : TransformStateType = TransformStateType::world_matrix(0);
    pub const World1        : TransformStateType = TransformStateType::world_matrix(1);
    pub const World2        : TransformStateType = TransformStateType::world_matrix(2);
    pub const World3        : TransformStateType = TransformStateType::world_matrix(3);
}

impl TransformStateType {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dts-worldmatrix)\]
    /// D3DTS_WORLDMATRIX
    pub const fn world_matrix(index: u8) -> TransformStateType { TransformStateType(index as u32 + 256) }
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for TransformStateType {
    fn default() -> Self { TransformStateType(0) }
}
