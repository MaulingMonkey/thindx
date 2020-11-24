#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureop)\]
/// D3DTEXTUREOP
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TextureOp(D3DTEXTUREOP);
pub type TOP = TextureOp;

impl TextureOp {
    /// Convert a raw [D3DTEXTUREOP] value into a [TextureOp].  This is *probably* safe... probably....
    ///
    /// [D3DTEXTUREOP]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureop
    pub const fn from_unchecked(textureop: D3DTEXTUREOP) -> Self { Self(textureop) }

    /// Convert a [TextureOp] into a raw [D3DTEXTUREOP].
    ///
    /// [D3DTEXTUREOP]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureop
    pub const fn into(self) -> D3DTEXTUREOP { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl TextureOp {
    pub const Disable                   : TextureOp = TextureOp(D3DTOP_DISABLE);
    pub const SelectArg1                : TextureOp = TextureOp(D3DTOP_SELECTARG1);
    pub const SelectArg2                : TextureOp = TextureOp(D3DTOP_SELECTARG2);
    pub const Modulate                  : TextureOp = TextureOp(D3DTOP_MODULATE);
    pub const Modulate2x                : TextureOp = TextureOp(D3DTOP_MODULATE2X);
    pub const Modulate4x                : TextureOp = TextureOp(D3DTOP_MODULATE4X);
    pub const Add                       : TextureOp = TextureOp(D3DTOP_ADD);
    pub const AddSigned                 : TextureOp = TextureOp(D3DTOP_ADDSIGNED);
    pub const AddSigned2x               : TextureOp = TextureOp(D3DTOP_ADDSIGNED2X);
    pub const Subtract                  : TextureOp = TextureOp(D3DTOP_SUBTRACT);
    pub const AddSmooth                 : TextureOp = TextureOp(D3DTOP_ADDSMOOTH);
    pub const BlendDiffuseAlpha         : TextureOp = TextureOp(D3DTOP_BLENDDIFFUSEALPHA);
    pub const BlendTextureAlpha         : TextureOp = TextureOp(D3DTOP_BLENDTEXTUREALPHA);
    pub const BlendFactorAlpha          : TextureOp = TextureOp(D3DTOP_BLENDFACTORALPHA);
    pub const BlendTextureAlphaPM       : TextureOp = TextureOp(D3DTOP_BLENDTEXTUREALPHAPM);
    pub const BlendCurrentAlpha         : TextureOp = TextureOp(D3DTOP_BLENDCURRENTALPHA);
    pub const PreModulate               : TextureOp = TextureOp(D3DTOP_PREMODULATE);
    pub const ModulateAlphaAddColor     : TextureOp = TextureOp(D3DTOP_MODULATEALPHA_ADDCOLOR);
    pub const ModulateColorAddAlpha     : TextureOp = TextureOp(D3DTOP_MODULATECOLOR_ADDALPHA);
    pub const ModulateInvAlphaAddColor  : TextureOp = TextureOp(D3DTOP_MODULATEINVALPHA_ADDCOLOR);
    pub const ModulateInvColorAddAlpha  : TextureOp = TextureOp(D3DTOP_MODULATEINVCOLOR_ADDALPHA);
    pub const BumpEnvMap                : TextureOp = TextureOp(D3DTOP_BUMPENVMAP);
    pub const BumpEnvMapLuminance       : TextureOp = TextureOp(D3DTOP_BUMPENVMAPLUMINANCE);
    pub const DotProduct3               : TextureOp = TextureOp(D3DTOP_DOTPRODUCT3);
    pub const MultiplyAdd               : TextureOp = TextureOp(D3DTOP_MULTIPLYADD);
    pub const Lerp                      : TextureOp = TextureOp(D3DTOP_LERP);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for TextureOp {
    fn default() -> Self { TextureOp(0) }
}

impl Debug for TextureOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TextureOp::Disable                  => write!(f, "TextureOp::Disable"),
            TextureOp::SelectArg1               => write!(f, "TextureOp::SelectArg1"),
            TextureOp::SelectArg2               => write!(f, "TextureOp::SelectArg2"),
            TextureOp::Modulate                 => write!(f, "TextureOp::Modulate"),
            TextureOp::Modulate2x               => write!(f, "TextureOp::Modulate2x"),
            TextureOp::Modulate4x               => write!(f, "TextureOp::Modulate4x"),
            TextureOp::Add                      => write!(f, "TextureOp::Add"),
            TextureOp::AddSigned                => write!(f, "TextureOp::AddSigned"),
            TextureOp::AddSigned2x              => write!(f, "TextureOp::AddSigned2x"),
            TextureOp::Subtract                 => write!(f, "TextureOp::Subtract"),
            TextureOp::AddSmooth                => write!(f, "TextureOp::AddSmooth"),
            TextureOp::BlendDiffuseAlpha        => write!(f, "TextureOp::BlendDiffuseAlpha"),
            TextureOp::BlendTextureAlpha        => write!(f, "TextureOp::BlendTextureAlpha"),
            TextureOp::BlendFactorAlpha         => write!(f, "TextureOp::BlendFactorAlpha"),
            TextureOp::BlendTextureAlphaPM      => write!(f, "TextureOp::BlendTextureAlphaPM"),
            TextureOp::BlendCurrentAlpha        => write!(f, "TextureOp::BlendCurrentAlpha"),
            TextureOp::PreModulate              => write!(f, "TextureOp::PreModulate"),
            TextureOp::ModulateAlphaAddColor    => write!(f, "TextureOp::ModulateAlphaAddColor"),
            TextureOp::ModulateColorAddAlpha    => write!(f, "TextureOp::ModulateColorAddAlpha"),
            TextureOp::ModulateInvAlphaAddColor => write!(f, "TextureOp::ModulateInvAlphaAddColor"),
            TextureOp::ModulateInvColorAddAlpha => write!(f, "TextureOp::ModulateInvColorAddAlpha"),
            TextureOp::BumpEnvMap               => write!(f, "TextureOp::BumpEnvMap"),
            TextureOp::BumpEnvMapLuminance      => write!(f, "TextureOp::BumpEnvMapLuminance"),
            TextureOp::DotProduct3              => write!(f, "TextureOp::DotProduct3"),
            TextureOp::MultiplyAdd              => write!(f, "TextureOp::MultiplyAdd"),
            TextureOp::Lerp                     => write!(f, "TextureOp::Lerp"),
            other                               => write!(f, "TextureOp({})", other.0),
        }
    }
}

impl From<TextureOp> for D3DTEXTUREOP {
    fn from(value: TextureOp) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DTEXTUREOP> for TextureOp {
    fn from(value: D3DTEXTUREOP) -> Self { Self(value) }
}
