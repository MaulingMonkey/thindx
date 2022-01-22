#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DPTEXTURECAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct PTAddressCaps(DWORD);

flags! {
    PTAddressCaps => DWORD;
    Border, Clamp, IndependentUV, Mirror, MirrorOnce, Wrap,
}

#[allow(non_upper_case_globals)] impl PTAddressCaps {
    pub const None                          : PTAddressCaps = PTAddressCaps(0);
    pub const Border                        : PTAddressCaps = PTAddressCaps(D3DPTADDRESSCAPS_BORDER);
    pub const Clamp                         : PTAddressCaps = PTAddressCaps(D3DPTADDRESSCAPS_CLAMP);
    pub const IndependentUV                 : PTAddressCaps = PTAddressCaps(D3DPTADDRESSCAPS_INDEPENDENTUV);
    pub const Mirror                        : PTAddressCaps = PTAddressCaps(D3DPTADDRESSCAPS_MIRROR);
    pub const MirrorOnce                    : PTAddressCaps = PTAddressCaps(D3DPTADDRESSCAPS_MIRRORONCE);
    pub const Wrap                          : PTAddressCaps = PTAddressCaps(D3DPTADDRESSCAPS_WRAP);
}

//#cpp2rust D3DPTADDRESSCAPS_BORDER         = d3d::PTAddressCaps::Border
//#cpp2rust D3DPTADDRESSCAPS_CLAMP          = d3d::PTAddressCaps::Clamp
//#cpp2rust D3DPTADDRESSCAPS_INDEPENDENTUV  = d3d::PTAddressCaps::IndependentUV
//#cpp2rust D3DPTADDRESSCAPS_MIRROR         = d3d::PTAddressCaps::Mirror
//#cpp2rust D3DPTADDRESSCAPS_MIRRORONCE     = d3d::PTAddressCaps::MirrorOnce
//#cpp2rust D3DPTADDRESSCAPS_WRAP           = d3d::PTAddressCaps::Wrap
