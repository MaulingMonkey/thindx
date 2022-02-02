#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureop)\]
/// D3DTEXTUREOP
///
/// Defines per-stage texture-blending operations.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct TextureOp(D3DTEXTUREOP);
pub use TextureOp as TOP;

enumish! {
    TOP => D3DTEXTUREOP;
    Disable, SelectArg1, SelectArg2, Modulate, Modulate2x, Modulate4x,
    Add, AddSigned, AddSigned2x, Subtract, AddSmooth,
    BlendDiffuseAlpha, BlendTextureAlpha, BlendFactorAlpha, BlendTextureAlphaPM, BlendCurrentAlpha,
    PreModulate, ModulateAlphaAddColor, ModulateColorAddAlpha, ModulateInvAlphaAddColor, ModulateInvColorAddAlpha,
    BumpEnvMap, BumpEnvMapLuminance, DotProduct3, MultiplyAdd, Lerp,
}

#[allow(non_upper_case_globals)] impl TextureOp { // These are enum-like
    pub const Disable                   : TextureOp = TextureOp(D3DTOP_DISABLE); // 1
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

impl TextureOp {
    pub const fn zeroed() -> Self { Self(0) }
}

//#cpp2rust D3DTEXTUREOP                        = d3d::TextureOp
//#cpp2rust D3DTOP_DISABLE                      = d3d::TOP::Disable
//#cpp2rust D3DTOP_SELECTARG1                   = d3d::TOP::SelectArg1
//#cpp2rust D3DTOP_SELECTARG2                   = d3d::TOP::SelectArg2
//#cpp2rust D3DTOP_MODULATE                     = d3d::TOP::Modulate
//#cpp2rust D3DTOP_MODULATE2X                   = d3d::TOP::Modulate2x
//#cpp2rust D3DTOP_MODULATE4X                   = d3d::TOP::Modulate4x
//#cpp2rust D3DTOP_ADD                          = d3d::TOP::Add
//#cpp2rust D3DTOP_ADDSIGNED                    = d3d::TOP::AddSigned
//#cpp2rust D3DTOP_ADDSIGNED2X                  = d3d::TOP::AddSigned2x
//#cpp2rust D3DTOP_SUBTRACT                     = d3d::TOP::Subtract
//#cpp2rust D3DTOP_ADDSMOOTH                    = d3d::TOP::AddSmooth
//#cpp2rust D3DTOP_BLENDDIFFUSEALPHA            = d3d::TOP::BlendDiffuseAlpha
//#cpp2rust D3DTOP_BLENDTEXTUREALPHA            = d3d::TOP::BlendTextureAlpha
//#cpp2rust D3DTOP_BLENDFACTORALPHA             = d3d::TOP::BlendFactorAlpha
//#cpp2rust D3DTOP_BLENDTEXTUREALPHAPM          = d3d::TOP::BlendTextureAlphaPM
//#cpp2rust D3DTOP_BLENDCURRENTALPHA            = d3d::TOP::BlendCurrentAlpha
//#cpp2rust D3DTOP_PREMODULATE                  = d3d::TOP::PreModulate
//#cpp2rust D3DTOP_MODULATEALPHA_ADDCOLOR       = d3d::TOP::ModulateAlphaAddColor
//#cpp2rust D3DTOP_MODULATECOLOR_ADDALPHA       = d3d::TOP::ModulateColorAddAlpha
//#cpp2rust D3DTOP_MODULATEINVALPHA_ADDCOLOR    = d3d::TOP::ModulateInvAlphaAddColor
//#cpp2rust D3DTOP_MODULATEINVCOLOR_ADDALPHA    = d3d::TOP::ModulateInvColorAddAlpha
//#cpp2rust D3DTOP_BUMPENVMAP                   = d3d::TOP::BumpEnvMap
//#cpp2rust D3DTOP_BUMPENVMAPLUMINANCE          = d3d::TOP::BumpEnvMapLuminance
//#cpp2rust D3DTOP_DOTPRODUCT3                  = d3d::TOP::DotProduct3
//#cpp2rust D3DTOP_MULTIPLYADD                  = d3d::TOP::MultiplyAdd
//#cpp2rust D3DTOP_LERP                         = d3d::TOP::Lerp
