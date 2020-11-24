#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtransformstatetype)\]
/// D3DTRANSFORMSTATETYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TransformStateType(D3DTRANSFORMSTATETYPE);

impl TransformStateType {
    /// Convert a raw [D3DTRANSFORMSTATETYPE] value into a [TransformStateType].  This is *probably* safe... probably....
    ///
    /// [D3DTRANSFORMSTATETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtransformstatetype
    pub const fn from_unchecked(transformstatetype: D3DTRANSFORMSTATETYPE) -> Self { Self(transformstatetype) }

    /// Convert a [TransformStateType] into a raw [D3DTRANSFORMSTATETYPE].
    ///
    /// [D3DTRANSFORMSTATETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtransformstatetype
    pub const fn into(self) -> D3DTRANSFORMSTATETYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl TransformStateType {
    pub const View          : TransformStateType = TransformStateType(D3DTS_VIEW);
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

impl Debug for TransformStateType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TransformStateType::View        => write!(f, "TransformStateType::View"),
            TransformStateType::Projection  => write!(f, "TransformStateType::Projection"),
            TransformStateType::Texture0    => write!(f, "TransformStateType::Texture0"),
            TransformStateType::Texture1    => write!(f, "TransformStateType::Texture1"),
            TransformStateType::Texture2    => write!(f, "TransformStateType::Texture2"),
            TransformStateType::Texture3    => write!(f, "TransformStateType::Texture3"),
            TransformStateType::Texture4    => write!(f, "TransformStateType::Texture4"),
            TransformStateType::Texture5    => write!(f, "TransformStateType::Texture5"),
            TransformStateType::Texture6    => write!(f, "TransformStateType::Texture6"),
            TransformStateType::Texture7    => write!(f, "TransformStateType::Texture7"),
            other                           => write!(f, "TransformStateType({})", other.0),
        }
    }
}

impl From<TransformStateType> for D3DTRANSFORMSTATETYPE {
    fn from(value: TransformStateType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DTRANSFORMSTATETYPE> for TransformStateType {
    fn from(value: D3DTRANSFORMSTATETYPE) -> Self { Self(value) }
}
