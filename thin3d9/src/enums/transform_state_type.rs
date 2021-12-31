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
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for TransformStateType {
    fn default() -> Self { TransformStateType(0) }
}
