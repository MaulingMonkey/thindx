#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturestagestatetype)\]
/// D3DTEXTURESTAGESTATETYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TextureStageStateType(D3DTEXTURESTAGESTATETYPE);
pub type TSS = TextureStageStateType;

impl TextureStageStateType {
    /// Convert a raw [D3DTEXTURESTAGESTATETYPE] value into a [TextureStageStateType].  This is *probably* safe... probably....
    ///
    /// [D3DTEXTURESTAGESTATETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturestagestatetype
    pub const fn from_unchecked(texturestagestatetype: D3DTEXTURESTAGESTATETYPE) -> Self { Self(texturestagestatetype) }

    /// Convert a [TextureStageStateType] into a raw [D3DTEXTURESTAGESTATETYPE].
    ///
    /// [D3DTEXTURESTAGESTATETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturestagestatetype
    pub const fn into(self) -> D3DTEXTURESTAGESTATETYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl TextureStageStateType {
    pub const ColorOp                   : TextureStageStateType = TextureStageStateType(D3DTSS_COLOROP);
    pub const ColorArg1                 : TextureStageStateType = TextureStageStateType(D3DTSS_COLORARG1);
    pub const ColorArg2                 : TextureStageStateType = TextureStageStateType(D3DTSS_COLORARG2);
    pub const AlphaOp                   : TextureStageStateType = TextureStageStateType(D3DTSS_ALPHAOP);
    pub const AlphaArg1                 : TextureStageStateType = TextureStageStateType(D3DTSS_ALPHAARG1);
    pub const AlphaArg2                 : TextureStageStateType = TextureStageStateType(D3DTSS_ALPHAARG2);
    pub const BumpEnvMat00              : TextureStageStateType = TextureStageStateType(D3DTSS_BUMPENVMAT00);
    pub const BumpEnvMat01              : TextureStageStateType = TextureStageStateType(D3DTSS_BUMPENVMAT01);
    pub const BumpEnvMat10              : TextureStageStateType = TextureStageStateType(D3DTSS_BUMPENVMAT10);
    pub const BumpEnvMat11              : TextureStageStateType = TextureStageStateType(D3DTSS_BUMPENVMAT11);
    pub const TexCoordIndex             : TextureStageStateType = TextureStageStateType(D3DTSS_TEXCOORDINDEX);
    pub const BumpEnvlScale             : TextureStageStateType = TextureStageStateType(D3DTSS_BUMPENVLSCALE);
    pub const BumpEnvlOffset            : TextureStageStateType = TextureStageStateType(D3DTSS_BUMPENVLOFFSET);
    pub const TextureTransformFlags     : TextureStageStateType = TextureStageStateType(D3DTSS_TEXTURETRANSFORMFLAGS);
    pub const ColorArg0                 : TextureStageStateType = TextureStageStateType(D3DTSS_COLORARG0);
    pub const AlphaArg0                 : TextureStageStateType = TextureStageStateType(D3DTSS_ALPHAARG0);
    pub const ResultArg                 : TextureStageStateType = TextureStageStateType(D3DTSS_RESULTARG);
    pub const Constant                  : TextureStageStateType = TextureStageStateType(D3DTSS_CONSTANT);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for TextureStageStateType {
    fn default() -> Self { TextureStageStateType(0) }
}

impl Debug for TextureStageStateType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TextureStageStateType::ColorOp                  => write!(f, "TextureStageStateType::ColorOp"),
            TextureStageStateType::ColorArg1                => write!(f, "TextureStageStateType::ColorArg1"),
            TextureStageStateType::ColorArg2                => write!(f, "TextureStageStateType::ColorArg2"),
            TextureStageStateType::AlphaOp                  => write!(f, "TextureStageStateType::AlphaOp"),
            TextureStageStateType::AlphaArg1                => write!(f, "TextureStageStateType::AlphaArg1"),
            TextureStageStateType::AlphaArg2                => write!(f, "TextureStageStateType::AlphaArg2"),
            TextureStageStateType::BumpEnvMat00             => write!(f, "TextureStageStateType::BumpEnvMat00"),
            TextureStageStateType::BumpEnvMat01             => write!(f, "TextureStageStateType::BumpEnvMat01"),
            TextureStageStateType::BumpEnvMat10             => write!(f, "TextureStageStateType::BumpEnvMat10"),
            TextureStageStateType::BumpEnvMat11             => write!(f, "TextureStageStateType::BumpEnvMat11"),
            TextureStageStateType::TexCoordIndex            => write!(f, "TextureStageStateType::TexCoordIndex"),
            TextureStageStateType::BumpEnvlScale            => write!(f, "TextureStageStateType::BumpEnvlScale"),
            TextureStageStateType::BumpEnvlOffset           => write!(f, "TextureStageStateType::BumpEnvlOffset"),
            TextureStageStateType::TextureTransformFlags    => write!(f, "TextureStageStateType::TextureTransformFlags"),
            TextureStageStateType::ColorArg0                => write!(f, "TextureStageStateType::ColorArg0"),
            TextureStageStateType::AlphaArg0                => write!(f, "TextureStageStateType::AlphaArg0"),
            TextureStageStateType::ResultArg                => write!(f, "TextureStageStateType::ResultArg"),
            TextureStageStateType::Constant                 => write!(f, "TextureStageStateType::Constant"),
            other                                           => write!(f, "TextureStageStateType({})", other.0),
        }
    }
}

impl From<TextureStageStateType> for D3DTEXTURESTAGESTATETYPE {
    fn from(value: TextureStageStateType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DTEXTURESTAGESTATETYPE> for TextureStageStateType {
    fn from(value: D3DTEXTURESTAGESTATETYPE) -> Self { Self(value) }
}
