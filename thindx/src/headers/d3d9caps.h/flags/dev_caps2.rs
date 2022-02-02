use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddevcaps2)\]
/// D3DDEVCAPS2_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct DevCaps2(DWORD);

flags! {
    DevCaps2 => DWORD;
    None, AdaptiveTessRtPatch, AdaptiveTessNPatch, CanStretchRectFromTextures, DMapNPatch, PresampledDMapNPatch, StreamOffset, VertexElementsCanShareStreamOffset,
}

#[allow(non_upper_case_globals)] impl DevCaps2 {
    pub const None                                  : DevCaps2 = DevCaps2(0);
    pub const AdaptiveTessRtPatch                   : DevCaps2 = DevCaps2(D3DDEVCAPS2_ADAPTIVETESSRTPATCH);
    pub const AdaptiveTessNPatch                    : DevCaps2 = DevCaps2(D3DDEVCAPS2_ADAPTIVETESSNPATCH);
    pub const CanStretchRectFromTextures            : DevCaps2 = DevCaps2(D3DDEVCAPS2_CAN_STRETCHRECT_FROM_TEXTURES);
    pub const DMapNPatch                            : DevCaps2 = DevCaps2(D3DDEVCAPS2_DMAPNPATCH);
    pub const PresampledDMapNPatch                  : DevCaps2 = DevCaps2(D3DDEVCAPS2_PRESAMPLEDDMAPNPATCH);
    pub const StreamOffset                          : DevCaps2 = DevCaps2(D3DDEVCAPS2_STREAMOFFSET);
    pub const VertexElementsCanShareStreamOffset    : DevCaps2 = DevCaps2(D3DDEVCAPS2_VERTEXELEMENTSCANSHARESTREAMOFFSET);
}

//#cpp2rust D3DDEVCAPS2_ADAPTIVETESSRTPATCH                 = d3d::DevCaps2::AdaptiveTessRtPatch
//#cpp2rust D3DDEVCAPS2_ADAPTIVETESSNPATCH                  = d3d::DevCaps2::AdaptiveTessNPatch
//#cpp2rust D3DDEVCAPS2_CAN_STRETCHRECT_FROM_TEXTURES       = d3d::DevCaps2::CanStretchRectFromTextures
//#cpp2rust D3DDEVCAPS2_DMAPNPATCH                          = d3d::DevCaps2::DMapNPatch
//#cpp2rust D3DDEVCAPS2_PRESAMPLEDDMAPNPATCH                = d3d::DevCaps2::PresampledDMapNPatch
//#cpp2rust D3DDEVCAPS2_STREAMOFFSET                        = d3d::DevCaps2::StreamOffset
//#cpp2rust D3DDEVCAPS2_VERTEXELEMENTSCANSHARESTREAMOFFSET  = d3d::DevCaps2::VertexElementsCanShareStreamOffset
