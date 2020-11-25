#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturestagestatetype)\]
/// D3DTEXTURESTAGESTATETYPE
///
/// Texture stage states define multi-blender texture operations.
/// Some sampler states set up vertex processing, and some set up pixel processing.
/// Texture stage states can be saved and restored using stateblocks (see [State Blocks Save and Restore State (Direct3D 9)]).
///
/// [State Blocks Save and Restore State (Direct3D 9)]:     https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TextureStageStateType(D3DTEXTURESTAGESTATETYPE);
pub type TSS = TextureStageStateType;

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

#[cfg(feature = "impl-poor-defaults")]
impl Default for TextureStageStateType {
    fn default() -> Self { TextureStageStateType(0) }
}
