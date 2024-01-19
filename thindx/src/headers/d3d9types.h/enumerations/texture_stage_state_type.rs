#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturestagestatetype)\]
/// D3DTEXTURESTAGESTATETYPE
///
/// Texture stage states define multi-blender texture operations.
/// Some sampler states set up vertex processing, and some set up pixel processing.
/// Texture stage states can be saved and restored using stateblocks (see [State Blocks Save and Restore State (Direct3D 9)]).
///
/// [State Blocks Save and Restore State (Direct3D 9)]:     https://learn.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct TextureStageStateType(D3DTEXTURESTAGESTATETYPE);
pub use TextureStageStateType as TSS;

enumish! {
    TSS => D3DTEXTURESTAGESTATETYPE;
    ColorOp, ColorArg1, ColorArg2, AlphaOp, AlphaArg1, AlphaArg2,
    BumpEnvMat00, BumpEnvMat01, BumpEnvMat10, BumpEnvMat11, TexCoordIndex, BumpEnvlScale, BumpEnvlOffset,
    TextureTransformFlags, ColorArg0, AlphaArg0, ResultArg, Constant,
}

#[allow(non_upper_case_globals)] impl TextureStageStateType { // These are enum-like
    pub const ColorOp                   : TextureStageStateType = TextureStageStateType(D3DTSS_COLOROP); // 1
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

//#cpp2rust D3DTEXTURESTAGESTATETYPE        = d3d::TextureStageStateType

//#cpp2rust D3DTSS_COLOROP                  = d3d::TSS::ColorOp
//#cpp2rust D3DTSS_COLORARG1                = d3d::TSS::ColorArg1
//#cpp2rust D3DTSS_COLORARG2                = d3d::TSS::ColorArg2
//#cpp2rust D3DTSS_ALPHAOP                  = d3d::TSS::AlphaOp
//#cpp2rust D3DTSS_ALPHAARG1                = d3d::TSS::AlphaArg1
//#cpp2rust D3DTSS_ALPHAARG2                = d3d::TSS::AlphaArg2
//#cpp2rust D3DTSS_BUMPENVMAT00             = d3d::TSS::BumpEnvMat00
//#cpp2rust D3DTSS_BUMPENVMAT01             = d3d::TSS::BumpEnvMat01
//#cpp2rust D3DTSS_BUMPENVMAT10             = d3d::TSS::BumpEnvMat10
//#cpp2rust D3DTSS_BUMPENVMAT11             = d3d::TSS::BumpEnvMat11
//#cpp2rust D3DTSS_TEXCOORDINDEX            = d3d::TSS::TexCoordIndex
//#cpp2rust D3DTSS_BUMPENVLSCALE            = d3d::TSS::BumpEnvlScale
//#cpp2rust D3DTSS_BUMPENVLOFFSET           = d3d::TSS::BumpEnvlOffset
//#cpp2rust D3DTSS_TEXTURETRANSFORMFLAGS    = d3d::TSS::TextureTransformFlags
//#cpp2rust D3DTSS_COLORARG0                = d3d::TSS::ColorArg0
//#cpp2rust D3DTSS_ALPHAARG0                = d3d::TSS::AlphaArg0
//#cpp2rust D3DTSS_RESULTARG                = d3d::TSS::ResultArg
//#cpp2rust D3DTSS_CONSTANT                 = d3d::TSS::Constant
