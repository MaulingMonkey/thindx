use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DDEVCAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct DevCaps(DWORD);

flags! {
    DevCaps => DWORD;
    None, CanBltSysToNonLocal, CanRenderAfterFlip, DrawPrimitives2, DrawPrimitives2Ex, DrawPrimTlVertex,
    ExecuteSystemMemory, ExecuteVideoMemory, HwRasterization, HwTransformAndLight, NPatches, PureDevice,
    QuinticRtPatches, RtPatches, RtPatchHandleZero, SeparateTextureMemories, TextureNonLocalVidMem,
    TextureSystemMemory, TextureVideoMemory, TlVertexSystemMemory, TlVertexVideoMemory
}

#[allow(non_upper_case_globals)] impl DevCaps {
    pub const None                          : DevCaps = DevCaps(0);
    pub const CanBltSysToNonLocal           : DevCaps = DevCaps(D3DDEVCAPS_CANBLTSYSTONONLOCAL);
    pub const CanRenderAfterFlip            : DevCaps = DevCaps(D3DDEVCAPS_CANRENDERAFTERFLIP);
    pub const DrawPrimitives2               : DevCaps = DevCaps(D3DDEVCAPS_DRAWPRIMITIVES2);
    pub const DrawPrimitives2Ex             : DevCaps = DevCaps(D3DDEVCAPS_DRAWPRIMITIVES2EX);
    pub const DrawPrimTlVertex              : DevCaps = DevCaps(D3DDEVCAPS_DRAWPRIMTLVERTEX);
    pub const ExecuteSystemMemory           : DevCaps = DevCaps(D3DDEVCAPS_EXECUTESYSTEMMEMORY);
    pub const ExecuteVideoMemory            : DevCaps = DevCaps(D3DDEVCAPS_EXECUTEVIDEOMEMORY);
    pub const HwRasterization               : DevCaps = DevCaps(D3DDEVCAPS_HWRASTERIZATION);
    pub const HwTransformAndLight           : DevCaps = DevCaps(D3DDEVCAPS_HWTRANSFORMANDLIGHT);
    pub const NPatches                      : DevCaps = DevCaps(D3DDEVCAPS_NPATCHES);
    pub const PureDevice                    : DevCaps = DevCaps(D3DDEVCAPS_PUREDEVICE);
    pub const QuinticRtPatches              : DevCaps = DevCaps(D3DDEVCAPS_QUINTICRTPATCHES);
    pub const RtPatches                     : DevCaps = DevCaps(D3DDEVCAPS_RTPATCHES);
    pub const RtPatchHandleZero             : DevCaps = DevCaps(D3DDEVCAPS_RTPATCHHANDLEZERO);
    pub const SeparateTextureMemories       : DevCaps = DevCaps(D3DDEVCAPS_SEPARATETEXTUREMEMORIES);
    pub const TextureNonLocalVidMem         : DevCaps = DevCaps(D3DDEVCAPS_TEXTURENONLOCALVIDMEM);
    pub const TextureSystemMemory           : DevCaps = DevCaps(D3DDEVCAPS_TEXTURESYSTEMMEMORY);
    pub const TextureVideoMemory            : DevCaps = DevCaps(D3DDEVCAPS_TEXTUREVIDEOMEMORY);
    pub const TlVertexSystemMemory          : DevCaps = DevCaps(D3DDEVCAPS_TLVERTEXSYSTEMMEMORY);
    pub const TlVertexVideoMemory           : DevCaps = DevCaps(D3DDEVCAPS_TLVERTEXVIDEOMEMORY);
}

//#cpp2rust D3DDEVCAPS_CANBLTSYSTONONLOCAL      = d3d::DevCaps::CanBltSysToNonLocal
//#cpp2rust D3DDEVCAPS_CANRENDERAFTERFLIP       = d3d::DevCaps::CanRenderAfterFlip
//#cpp2rust D3DDEVCAPS_DRAWPRIMITIVES2          = d3d::DevCaps::DrawPrimitives2
//#cpp2rust D3DDEVCAPS_DRAWPRIMITIVES2EX        = d3d::DevCaps::DrawPrimitives2Ex
//#cpp2rust D3DDEVCAPS_DRAWPRIMTLVERTEX         = d3d::DevCaps::DrawPrimTlVertex
//#cpp2rust D3DDEVCAPS_EXECUTESYSTEMMEMORY      = d3d::DevCaps::ExecuteSystemMemory
//#cpp2rust D3DDEVCAPS_EXECUTEVIDEOMEMORY       = d3d::DevCaps::ExecuteVideoMemory
//#cpp2rust D3DDEVCAPS_HWRASTERIZATION          = d3d::DevCaps::HwRasterization
//#cpp2rust D3DDEVCAPS_HWTRANSFORMANDLIGHT      = d3d::DevCaps::HwTransformAndLight
//#cpp2rust D3DDEVCAPS_NPATCHES                 = d3d::DevCaps::NPatches
//#cpp2rust D3DDEVCAPS_PUREDEVICE               = d3d::DevCaps::PureDevice
//#cpp2rust D3DDEVCAPS_QUINTICRTPATCHES         = d3d::DevCaps::QuinticRtPatches
//#cpp2rust D3DDEVCAPS_RTPATCHES                = d3d::DevCaps::RtPatches
//#cpp2rust D3DDEVCAPS_RTPATCHHANDLEZERO        = d3d::DevCaps::RtPatchHandleZero
//#cpp2rust D3DDEVCAPS_SEPARATETEXTUREMEMORIES  = d3d::DevCaps::SeparateTextureMemories
//#cpp2rust D3DDEVCAPS_TEXTURENONLOCALVIDMEM    = d3d::DevCaps::TextureNonLocalVidMem
//#cpp2rust D3DDEVCAPS_TEXTURESYSTEMMEMORY      = d3d::DevCaps::TextureSystemMemory
//#cpp2rust D3DDEVCAPS_TEXTUREVIDEOMEMORY       = d3d::DevCaps::TextureVideoMemory
//#cpp2rust D3DDEVCAPS_TLVERTEXSYSTEMMEMORY     = d3d::DevCaps::TlVertexSystemMemory
//#cpp2rust D3DDEVCAPS_TLVERTEXVIDEOMEMORY      = d3d::DevCaps::TlVertexVideoMemory
