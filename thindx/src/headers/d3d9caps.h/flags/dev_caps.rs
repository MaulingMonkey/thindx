use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DDEVCAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct DevCaps(DWORD);

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
