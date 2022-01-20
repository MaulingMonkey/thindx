use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DTEXOPCAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct TexOpCaps(DWORD);

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
