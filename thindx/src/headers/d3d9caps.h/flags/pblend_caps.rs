use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DPBLENDCAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct PBlendCaps(DWORD);

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
