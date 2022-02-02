use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DPBLENDCAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct PBlendCaps(DWORD);

flags! {
    PBlendCaps => DWORD;
    None, BlendFactor, BothInvSrcAlpha, BothSrcAlpha, DestAlpha, DestColor, InvDestAlpha, InvDestColor,
    InvSrcAlpha, InvSrcColor, InvSrcColor2, One, SrcAlpha, SrcAlphaSat, SrcColor, SrcColor2, Zero,
}

#[allow(non_upper_case_globals)] impl PBlendCaps {
    pub const None                          : PBlendCaps = PBlendCaps(0);
    pub const BlendFactor                   : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_BLENDFACTOR);
    pub const BothInvSrcAlpha               : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_BOTHINVSRCALPHA);
    pub const BothSrcAlpha                  : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_BOTHSRCALPHA);
    pub const DestAlpha                     : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_DESTALPHA);
    pub const DestColor                     : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_DESTCOLOR);
    pub const InvDestAlpha                  : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_INVDESTALPHA);
    pub const InvDestColor                  : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_INVDESTCOLOR);
    pub const InvSrcAlpha                   : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_INVSRCALPHA);
    pub const InvSrcColor                   : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_INVSRCCOLOR);
    pub const InvSrcColor2                  : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_INVSRCCOLOR2);
    pub const One                           : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_ONE);
    pub const SrcAlpha                      : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_SRCALPHA);
    pub const SrcAlphaSat                   : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_SRCALPHASAT);
    pub const SrcColor                      : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_SRCCOLOR);
    pub const SrcColor2                     : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_SRCCOLOR2);
    pub const Zero                          : PBlendCaps = PBlendCaps(D3DPBLENDCAPS_ZERO);
}

//#cpp2rust D3DPBLENDCAPS_BLENDFACTOR       = d3d::PBlendCaps::BlendFactor
//#cpp2rust D3DPBLENDCAPS_BOTHINVSRCALPHA   = d3d::PBlendCaps::BothInvSrcAlpha
//#cpp2rust D3DPBLENDCAPS_BOTHSRCALPHA      = d3d::PBlendCaps::BothSrcAlpha
//#cpp2rust D3DPBLENDCAPS_DESTALPHA         = d3d::PBlendCaps::DestAlpha
//#cpp2rust D3DPBLENDCAPS_DESTCOLOR         = d3d::PBlendCaps::DestColor
//#cpp2rust D3DPBLENDCAPS_INVDESTALPHA      = d3d::PBlendCaps::InvDestAlpha
//#cpp2rust D3DPBLENDCAPS_INVDESTCOLOR      = d3d::PBlendCaps::InvDestColor
//#cpp2rust D3DPBLENDCAPS_INVSRCALPHA       = d3d::PBlendCaps::InvSrcAlpha
//#cpp2rust D3DPBLENDCAPS_INVSRCCOLOR       = d3d::PBlendCaps::InvSrcColor
//#cpp2rust D3DPBLENDCAPS_INVSRCCOLOR2      = d3d::PBlendCaps::InvSrcColor2
//#cpp2rust D3DPBLENDCAPS_ONE               = d3d::PBlendCaps::One
//#cpp2rust D3DPBLENDCAPS_SRCALPHA          = d3d::PBlendCaps::SrcAlpha
//#cpp2rust D3DPBLENDCAPS_SRCALPHASAT       = d3d::PBlendCaps::SrcAlphaSat
//#cpp2rust D3DPBLENDCAPS_SRCCOLOR          = d3d::PBlendCaps::SrcColor
//#cpp2rust D3DPBLENDCAPS_SRCCOLOR2         = d3d::PBlendCaps::SrcColor2
//#cpp2rust D3DPBLENDCAPS_ZERO              = d3d::PBlendCaps::Zero
