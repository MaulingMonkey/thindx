use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DLINECAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct LineCaps(DWORD);

flags! {
    LineCaps => DWORD;
    None, AlphaCmp, AntiAlias, Blend, Fog, Texture, ZTest,
}

#[allow(non_upper_case_globals)] impl LineCaps {
    pub const None                          : LineCaps = LineCaps(0);
    pub const AlphaCmp                      : LineCaps = LineCaps(D3DLINECAPS_ALPHACMP);
    pub const AntiAlias                     : LineCaps = LineCaps(D3DLINECAPS_ANTIALIAS);
    pub const Blend                         : LineCaps = LineCaps(D3DLINECAPS_BLEND);
    pub const Fog                           : LineCaps = LineCaps(D3DLINECAPS_FOG);
    pub const Texture                       : LineCaps = LineCaps(D3DLINECAPS_TEXTURE);
    pub const ZTest                         : LineCaps = LineCaps(D3DLINECAPS_ZTEST);
}

//#cpp2rust D3DLINECAPS_ALPHACMP    = d3d::LineCaps::AlphaCmp
//#cpp2rust D3DLINECAPS_ANTIALIAS   = d3d::LineCaps::AntiAlias
//#cpp2rust D3DLINECAPS_BLEND       = d3d::LineCaps::Blend
//#cpp2rust D3DLINECAPS_FOG         = d3d::LineCaps::Fog
//#cpp2rust D3DLINECAPS_TEXTURE     = d3d::LineCaps::Texture
//#cpp2rust D3DLINECAPS_ZTEST       = d3d::LineCaps::ZTest
