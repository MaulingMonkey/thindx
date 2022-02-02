use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DTEXOPCAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct TexOpCaps(DWORD);

flags! {
    TexOpCaps => DWORD;
    None, Add, AddSigned, AddSigned2x, AddSmooth, BlendCurrentAlpha, BlendDiffuseAlpha, BlendFactorAlpha, BlendTextureAlpha,
    BlendTextureAlphaPM, BumpEnvMap, BumpEnvMapLuminance, Disable, DotProduct3, Lerp, Modulate, Modulate2x, Modulate4x,
    ModulateAlphaAddColor, ModulateColorAddAlpha, ModulateInvAlphaAddColor, ModulateInvColorAddAlpha, MultiplyAdd,
    Premodulate, SelectArg1, SelectArg2, Subtract,
}

#[allow(non_upper_case_globals)] impl TexOpCaps {
    pub const None                          : TexOpCaps = TexOpCaps(0);
    pub const Add                           : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_ADD);
    pub const AddSigned                     : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_ADDSIGNED);
    pub const AddSigned2x                   : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_ADDSIGNED2X);
    pub const AddSmooth                     : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_ADDSMOOTH);
    pub const BlendCurrentAlpha             : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_BLENDCURRENTALPHA);
    pub const BlendDiffuseAlpha             : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_BLENDDIFFUSEALPHA);
    pub const BlendFactorAlpha              : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_BLENDFACTORALPHA);
    pub const BlendTextureAlpha             : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_BLENDTEXTUREALPHA);
    pub const BlendTextureAlphaPM           : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_BLENDTEXTUREALPHAPM);
    pub const BumpEnvMap                    : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_BUMPENVMAP);
    pub const BumpEnvMapLuminance           : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_BUMPENVMAPLUMINANCE);
    pub const Disable                       : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_DISABLE);
    pub const DotProduct3                   : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_DOTPRODUCT3);
    pub const Lerp                          : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_LERP);
    pub const Modulate                      : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_MODULATE);
    pub const Modulate2x                    : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_MODULATE2X);
    pub const Modulate4x                    : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_MODULATE4X);
    pub const ModulateAlphaAddColor         : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_MODULATEALPHA_ADDCOLOR);
    pub const ModulateColorAddAlpha         : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_MODULATECOLOR_ADDALPHA);
    pub const ModulateInvAlphaAddColor      : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_MODULATEINVALPHA_ADDCOLOR);
    pub const ModulateInvColorAddAlpha      : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_MODULATEINVCOLOR_ADDALPHA);
    pub const MultiplyAdd                   : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_MULTIPLYADD);
    pub const Premodulate                   : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_PREMODULATE);
    pub const SelectArg1                    : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_SELECTARG1);
    pub const SelectArg2                    : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_SELECTARG2);
    pub const Subtract                      : TexOpCaps = TexOpCaps(D3DTEXOPCAPS_SUBTRACT);
}

//#cpp2rust D3DTEXOPCAPS_ADD                        = d3d::TexOpCaps::Add
//#cpp2rust D3DTEXOPCAPS_ADDSIGNED                  = d3d::TexOpCaps::AddSigned
//#cpp2rust D3DTEXOPCAPS_ADDSIGNED2X                = d3d::TexOpCaps::AddSigned2x
//#cpp2rust D3DTEXOPCAPS_ADDSMOOTH                  = d3d::TexOpCaps::AddSmooth
//#cpp2rust D3DTEXOPCAPS_BLENDCURRENTALPHA          = d3d::TexOpCaps::BlendCurrentAlpha
//#cpp2rust D3DTEXOPCAPS_BLENDDIFFUSEALPHA          = d3d::TexOpCaps::BlendDiffuseAlpha
//#cpp2rust D3DTEXOPCAPS_BLENDFACTORALPHA           = d3d::TexOpCaps::BlendFactorAlpha
//#cpp2rust D3DTEXOPCAPS_BLENDTEXTUREALPHA          = d3d::TexOpCaps::BlendTextureAlpha
//#cpp2rust D3DTEXOPCAPS_BLENDTEXTUREALPHAPM        = d3d::TexOpCaps::BlendTextureAlphaPM
//#cpp2rust D3DTEXOPCAPS_BUMPENVMAP                 = d3d::TexOpCaps::BumpEnvMap
//#cpp2rust D3DTEXOPCAPS_BUMPENVMAPLUMINANCE        = d3d::TexOpCaps::BumpEnvMapLuminance
//#cpp2rust D3DTEXOPCAPS_DISABLE                    = d3d::TexOpCaps::Disable
//#cpp2rust D3DTEXOPCAPS_DOTPRODUCT3                = d3d::TexOpCaps::DotProduct3
//#cpp2rust D3DTEXOPCAPS_LERP                       = d3d::TexOpCaps::Lerp
//#cpp2rust D3DTEXOPCAPS_MODULATE                   = d3d::TexOpCaps::Modulate
//#cpp2rust D3DTEXOPCAPS_MODULATE2X                 = d3d::TexOpCaps::Modulate2x
//#cpp2rust D3DTEXOPCAPS_MODULATE4X                 = d3d::TexOpCaps::Modulate4x
//#cpp2rust D3DTEXOPCAPS_MODULATEALPHA_ADDCOLOR     = d3d::TexOpCaps::ModulateAlphaAddColor
//#cpp2rust D3DTEXOPCAPS_MODULATECOLOR_ADDALPHA     = d3d::TexOpCaps::ModulateColorAddAlpha
//#cpp2rust D3DTEXOPCAPS_MODULATEINVALPHA_ADDCOLOR  = d3d::TexOpCaps::ModulateInvAlphaAddColor
//#cpp2rust D3DTEXOPCAPS_MODULATEINVCOLOR_ADDALPHA  = d3d::TexOpCaps::ModulateInvColorAddAlpha
//#cpp2rust D3DTEXOPCAPS_MULTIPLYADD                = d3d::TexOpCaps::MultiplyAdd
//#cpp2rust D3DTEXOPCAPS_PREMODULATE                = d3d::TexOpCaps::Premodulate
//#cpp2rust D3DTEXOPCAPS_SELECTARG1                 = d3d::TexOpCaps::SelectArg1
//#cpp2rust D3DTEXOPCAPS_SELECTARG2                 = d3d::TexOpCaps::SelectArg2
//#cpp2rust D3DTEXOPCAPS_SUBTRACT                   = d3d::TexOpCaps::Subtract
