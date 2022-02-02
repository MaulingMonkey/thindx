use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpmisccaps)\]
/// D3DPMISCCAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct PMiscCaps(DWORD);

flags! {
    PMiscCaps => DWORD;
    None, MaskZ, CullNone, CullCW, CullCCW, ColorWriteEnable, ClipPlaneScaledPoints, ClipTlVerts, TssArgTemp, BlendOp,
    NullReference, IndependentWriteMasks, PerStageConstant, PostBlendSrgbConvert, FogAndSpecularAlpha,
    SeparateAlphaBlend, MrtIndependentBitDepths, MrtPostPixelShaderBlending, FogVertexClamped,
}

#[allow(non_upper_case_globals)] impl PMiscCaps {
    pub const None                              : PMiscCaps = PMiscCaps(0);
    pub const MaskZ                             : PMiscCaps = PMiscCaps(D3DPMISCCAPS_MASKZ);
    pub const CullNone                          : PMiscCaps = PMiscCaps(D3DPMISCCAPS_CULLNONE);
    pub const CullCW                            : PMiscCaps = PMiscCaps(D3DPMISCCAPS_CULLCW);
    pub const CullCCW                           : PMiscCaps = PMiscCaps(D3DPMISCCAPS_CULLCCW);
    pub const ColorWriteEnable                  : PMiscCaps = PMiscCaps(D3DPMISCCAPS_COLORWRITEENABLE);
    pub const ClipPlaneScaledPoints             : PMiscCaps = PMiscCaps(D3DPMISCCAPS_CLIPPLANESCALEDPOINTS);
    pub const ClipTlVerts                       : PMiscCaps = PMiscCaps(D3DPMISCCAPS_CLIPTLVERTS);
    pub const TssArgTemp                        : PMiscCaps = PMiscCaps(D3DPMISCCAPS_TSSARGTEMP);
    pub const BlendOp                           : PMiscCaps = PMiscCaps(D3DPMISCCAPS_BLENDOP);
    pub const NullReference                     : PMiscCaps = PMiscCaps(D3DPMISCCAPS_NULLREFERENCE);
    pub const IndependentWriteMasks             : PMiscCaps = PMiscCaps(D3DPMISCCAPS_INDEPENDENTWRITEMASKS);
    pub const PerStageConstant                  : PMiscCaps = PMiscCaps(D3DPMISCCAPS_PERSTAGECONSTANT);
    pub const PostBlendSrgbConvert              : PMiscCaps = PMiscCaps(D3DPMISCCAPS_POSTBLENDSRGBCONVERT);
    pub const FogAndSpecularAlpha               : PMiscCaps = PMiscCaps(D3DPMISCCAPS_FOGANDSPECULARALPHA);
    pub const SeparateAlphaBlend                : PMiscCaps = PMiscCaps(D3DPMISCCAPS_SEPARATEALPHABLEND);
    pub const MrtIndependentBitDepths           : PMiscCaps = PMiscCaps(D3DPMISCCAPS_MRTINDEPENDENTBITDEPTHS);
    pub const MrtPostPixelShaderBlending        : PMiscCaps = PMiscCaps(D3DPMISCCAPS_MRTPOSTPIXELSHADERBLENDING);
    pub const FogVertexClamped                  : PMiscCaps = PMiscCaps(D3DPMISCCAPS_FOGVERTEXCLAMPED);
}

//#cpp2rust D3DPMISCCAPS_MASKZ                              = d3d::PMiscCaps::MaskZ
//#cpp2rust D3DPMISCCAPS_CULLNONE                           = d3d::PMiscCaps::CullNone
//#cpp2rust D3DPMISCCAPS_CULLCW                             = d3d::PMiscCaps::CullCW
//#cpp2rust D3DPMISCCAPS_CULLCCW                            = d3d::PMiscCaps::CullCCW
//#cpp2rust D3DPMISCCAPS_COLORWRITEENABLE                   = d3d::PMiscCaps::ColorWriteEnable
//#cpp2rust D3DPMISCCAPS_CLIPPLANESCALEDPOINTS              = d3d::PMiscCaps::ClipPlaneScaledPoints
//#cpp2rust D3DPMISCCAPS_CLIPTLVERTS                        = d3d::PMiscCaps::ClipTlVerts
//#cpp2rust D3DPMISCCAPS_TSSARGTEMP                         = d3d::PMiscCaps::TssArgTemp
//#cpp2rust D3DPMISCCAPS_BLENDOP                            = d3d::PMiscCaps::BlendOp
//#cpp2rust D3DPMISCCAPS_NULLREFERENCE                      = d3d::PMiscCaps::NullReference
//#cpp2rust D3DPMISCCAPS_INDEPENDENTWRITEMASKS              = d3d::PMiscCaps::IndependentWriteMasks
//#cpp2rust D3DPMISCCAPS_PERSTAGECONSTANT                   = d3d::PMiscCaps::PerStageConstant
//#cpp2rust D3DPMISCCAPS_POSTBLENDSRGBCONVERT               = d3d::PMiscCaps::PostBlendSrgbConvert
//#cpp2rust D3DPMISCCAPS_FOGANDSPECULARALPHA                = d3d::PMiscCaps::FogAndSpecularAlpha
//#cpp2rust D3DPMISCCAPS_SEPARATEALPHABLEND                 = d3d::PMiscCaps::SeparateAlphaBlend
//#cpp2rust D3DPMISCCAPS_MRTINDEPENDENTBITDEPTHS            = d3d::PMiscCaps::MrtIndependentBitDepths
//#cpp2rust D3DPMISCCAPS_MRTPOSTPIXELSHADERBLENDING         = d3d::PMiscCaps::MrtPostPixelShaderBlending
//#cpp2rust D3DPMISCCAPS_FOGVERTEXCLAMPED                   = d3d::PMiscCaps::FogVertexClamped
