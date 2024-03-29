use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;
//use winapi::um::d3dcaps::*; // does not exist



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DPRASTERCAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct PRasterCaps(DWORD);

flags! {
    PRasterCaps => DWORD;
    None, Anisotropy, ColorPerspective, Dither, DepthBias, FogRange, FogTable, FogVertex, MipMapLodBias,
    MultisampleToggle, ScissorTest, SlopeScaleDepthBias, WBuffer, WFog, ZBufferLessHSR, ZFog, ZTest,
    // d3dcaps.h
    Rop2, Xor, Pat, SubPixel, SubPixelX, Stipple, AntiAliasSortDependent,
    AntiAliasSortIndependent, AntiAliasEdges, ZBias, TranslucentSortIndependent,
}

#[allow(non_upper_case_globals)] impl PRasterCaps {
    pub const None                          : PRasterCaps = PRasterCaps(0);
    pub const Anisotropy                    : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_ANISOTROPY);
    pub const ColorPerspective              : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_COLORPERSPECTIVE);
    pub const Dither                        : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_DITHER);
    pub const DepthBias                     : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_DEPTHBIAS);
    pub const FogRange                      : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_FOGRANGE);
    pub const FogTable                      : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_FOGTABLE);
    pub const FogVertex                     : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_FOGVERTEX);
    pub const MipMapLodBias                 : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_MIPMAPLODBIAS);
    pub const MultisampleToggle             : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_MULTISAMPLE_TOGGLE);
    pub const ScissorTest                   : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_SCISSORTEST);
    pub const SlopeScaleDepthBias           : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_SLOPESCALEDEPTHBIAS);
    pub const WBuffer                       : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_WBUFFER);
    pub const WFog                          : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_WFOG);
    pub const ZBufferLessHSR                : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_ZBUFFERLESSHSR);
    pub const ZFog                          : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_ZFOG);
    pub const ZTest                         : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_ZTEST);

    // d3dcaps.h

    pub const Rop2                          : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_ROP2);
    pub const Xor                           : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_XOR);
    pub const Pat                           : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_PAT);
    pub const SubPixel                      : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_SUBPIXEL);
    pub const SubPixelX                     : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_SUBPIXELX);
    pub const Stipple                       : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_STIPPLE);
    pub const AntiAliasSortDependent        : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_ANTIALIASSORTDEPENDENT);
    pub const AntiAliasSortIndependent      : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_ANTIALIASSORTINDEPENDENT);
    pub const AntiAliasEdges                : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_ANTIALIASEDGES);
    pub const ZBias                         : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_ZBIAS);
    pub const TranslucentSortIndependent    : PRasterCaps = PRasterCaps(D3DPRASTERCAPS_TRANSLUCENTSORTINDEPENDENT);
}



// um/d3dcaps.h - many of these aren't defined in d3d9caps.h, but some are still reported by the device
// (notably D3DPRASTERCAPS_ZBIAS on my machine!)
// redundantly defined constants are left commented out

//const D3DPRASTERCAPS_DITHER                       : u32 = 0x00000001;
const D3DPRASTERCAPS_ROP2                           : u32 = 0x00000002;
const D3DPRASTERCAPS_XOR                            : u32 = 0x00000004;
const D3DPRASTERCAPS_PAT                            : u32 = 0x00000008;
//const D3DPRASTERCAPS_ZTEST                        : u32 = 0x00000010;
const D3DPRASTERCAPS_SUBPIXEL                       : u32 = 0x00000020;
const D3DPRASTERCAPS_SUBPIXELX                      : u32 = 0x00000040;
//const D3DPRASTERCAPS_FOGVERTEX                    : u32 = 0x00000080;
//const D3DPRASTERCAPS_FOGTABLE                     : u32 = 0x00000100;
const D3DPRASTERCAPS_STIPPLE                        : u32 = 0x00000200;
//#if(DIRECT3D_VERSION >= 0x0500)
const D3DPRASTERCAPS_ANTIALIASSORTDEPENDENT         : u32 = 0x00000400;
const D3DPRASTERCAPS_ANTIALIASSORTINDEPENDENT       : u32 = 0x00000800;
const D3DPRASTERCAPS_ANTIALIASEDGES                 : u32 = 0x00001000;
//const D3DPRASTERCAPS_MIPMAPLODBIAS                : u32 = 0x00002000;
const D3DPRASTERCAPS_ZBIAS                          : u32 = 0x00004000;
//const D3DPRASTERCAPS_ZBUFFERLESSHSR               : u32 = 0x00008000;
//const D3DPRASTERCAPS_FOGRANGE                     : u32 = 0x00010000;
//const D3DPRASTERCAPS_ANISOTROPY                   : u32 = 0x00020000;
//#endif /* DIRECT3D_VERSION >= 0x0500 */
//#if(DIRECT3D_VERSION >= 0x0600)
//const D3DPRASTERCAPS_WBUFFER                      : u32 = 0x00040000;
const D3DPRASTERCAPS_TRANSLUCENTSORTINDEPENDENT     : u32 = 0x00080000;
//const D3DPRASTERCAPS_WFOG                         : u32 = 0x00100000;
//const D3DPRASTERCAPS_ZFOG                         : u32 = 0x00200000;
//#endif /* DIRECT3D_VERSION >= 0x0600 */



//#cpp2rust D3DPRASTERCAPS_ANISOTROPY                   = d3d::PRasterCaps::Anisotropy
//#cpp2rust D3DPRASTERCAPS_COLORPERSPECTIVE             = d3d::PRasterCaps::ColorPerspective
//#cpp2rust D3DPRASTERCAPS_DITHER                       = d3d::PRasterCaps::Dither
//#cpp2rust D3DPRASTERCAPS_DEPTHBIAS                    = d3d::PRasterCaps::DepthBias
//#cpp2rust D3DPRASTERCAPS_FOGRANGE                     = d3d::PRasterCaps::FogRange
//#cpp2rust D3DPRASTERCAPS_FOGTABLE                     = d3d::PRasterCaps::FogTable
//#cpp2rust D3DPRASTERCAPS_FOGVERTEX                    = d3d::PRasterCaps::FogVertex
//#cpp2rust D3DPRASTERCAPS_MIPMAPLODBIAS                = d3d::PRasterCaps::MipMapLodBias
//#cpp2rust D3DPRASTERCAPS_MULTISAMPLE_TOGGLE           = d3d::PRasterCaps::MultisampleToggle
//#cpp2rust D3DPRASTERCAPS_SCISSORTEST                  = d3d::PRasterCaps::ScissorTest
//#cpp2rust D3DPRASTERCAPS_SLOPESCALEDEPTHBIAS          = d3d::PRasterCaps::SlopeScaleDepthBias
//#cpp2rust D3DPRASTERCAPS_WBUFFER                      = d3d::PRasterCaps::WBuffer
//#cpp2rust D3DPRASTERCAPS_WFOG                         = d3d::PRasterCaps::WFog
//#cpp2rust D3DPRASTERCAPS_ZBUFFERLESSHSR               = d3d::PRasterCaps::ZBufferLessHSR
//#cpp2rust D3DPRASTERCAPS_ZFOG                         = d3d::PRasterCaps::ZFog
//#cpp2rust D3DPRASTERCAPS_ZTEST                        = d3d::PRasterCaps::ZTest

//#cpp2rust D3DPRASTERCAPS_ROP2                         = d3d::PRasterCaps::Rop2
//#cpp2rust D3DPRASTERCAPS_XOR                          = d3d::PRasterCaps::Xor
//#cpp2rust D3DPRASTERCAPS_PAT                          = d3d::PRasterCaps::Pat
//#cpp2rust D3DPRASTERCAPS_SUBPIXEL                     = d3d::PRasterCaps::SubPixel
//#cpp2rust D3DPRASTERCAPS_SUBPIXELX                    = d3d::PRasterCaps::SubPixelX
//#cpp2rust D3DPRASTERCAPS_STIPPLE                      = d3d::PRasterCaps::Stipple
//#cpp2rust D3DPRASTERCAPS_ANTIALIASSORTDEPENDENT       = d3d::PRasterCaps::AntiAliasSortDependent
//#cpp2rust D3DPRASTERCAPS_ANTIALIASSORTINDEPENDENT     = d3d::PRasterCaps::AntiAliasSortIndependent
//#cpp2rust D3DPRASTERCAPS_ANTIALIASEDGES               = d3d::PRasterCaps::AntiAliasEdges
//#cpp2rust D3DPRASTERCAPS_ZBIAS                        = d3d::PRasterCaps::ZBias
//#cpp2rust D3DPRASTERCAPS_TRANSLUCENTSORTINDEPENDENT   = d3d::PRasterCaps::TranslucentSortIndependent
