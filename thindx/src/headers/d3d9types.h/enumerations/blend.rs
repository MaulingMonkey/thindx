#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblend)\]
/// D3DBLEND
///
/// Defines the supported blend mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Blend(D3DBLEND);

enumish! {
    Blend => D3DBLEND;
    Zero, One, SrcColor, InvSrcColor, SrcAlpha, InvSrcAlpha, DestAlpha, InvDestAlpha, DestColor, InvDestColor,
    SrcAlphaSat, BothSrcAlpha, BothInvSrcAlpha, BlendFactor, InvBlendFactor, SrcColor2, InvSrcColor2,
}

#[allow(non_upper_case_globals)] impl Blend { // These are enum-like
    pub const Zero                  : Blend = Blend(D3DBLEND_ZERO); // 1 - ironic, I know
    pub const One                   : Blend = Blend(D3DBLEND_ONE);  // 2 - herp derp
    pub const SrcColor              : Blend = Blend(D3DBLEND_SRCCOLOR);
    pub const InvSrcColor           : Blend = Blend(D3DBLEND_INVSRCCOLOR);
    pub const SrcAlpha              : Blend = Blend(D3DBLEND_SRCALPHA);
    pub const InvSrcAlpha           : Blend = Blend(D3DBLEND_INVSRCALPHA);
    pub const DestAlpha             : Blend = Blend(D3DBLEND_DESTALPHA);
    pub const InvDestAlpha          : Blend = Blend(D3DBLEND_INVDESTALPHA);
    pub const DestColor             : Blend = Blend(D3DBLEND_DESTCOLOR);
    pub const InvDestColor          : Blend = Blend(D3DBLEND_INVDESTCOLOR);
    pub const SrcAlphaSat           : Blend = Blend(D3DBLEND_SRCALPHASAT);
    pub const BothSrcAlpha          : Blend = Blend(D3DBLEND_BOTHSRCALPHA);
    pub const BothInvSrcAlpha       : Blend = Blend(D3DBLEND_BOTHINVSRCALPHA);
    pub const BlendFactor           : Blend = Blend(D3DBLEND_BLENDFACTOR);
    pub const InvBlendFactor        : Blend = Blend(D3DBLEND_INVBLENDFACTOR);
    pub const SrcColor2             : Blend = Blend(D3DBLEND_SRCCOLOR2);
    pub const InvSrcColor2          : Blend = Blend(D3DBLEND_INVSRCCOLOR2);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for Blend {
    fn default() -> Self { Blend(0) }
}

//#cpp2rust D3DBLEND                    = d3d::Blend
//#cpp2rust D3DBLEND_ZERO               = d3d::Blend::Zero
//#cpp2rust D3DBLEND_ONE                = d3d::Blend::One
//#cpp2rust D3DBLEND_SRCCOLOR           = d3d::Blend::SrcColor
//#cpp2rust D3DBLEND_INVSRCCOLOR        = d3d::Blend::InvSrcColor
//#cpp2rust D3DBLEND_SRCALPHA           = d3d::Blend::SrcAlpha
//#cpp2rust D3DBLEND_INVSRCALPHA        = d3d::Blend::InvSrcAlpha
//#cpp2rust D3DBLEND_DESTALPHA          = d3d::Blend::DestAlpha
//#cpp2rust D3DBLEND_INVDESTALPHA       = d3d::Blend::InvDestAlpha
//#cpp2rust D3DBLEND_DESTCOLOR          = d3d::Blend::DestColor
//#cpp2rust D3DBLEND_INVDESTCOLOR       = d3d::Blend::InvDestColor
//#cpp2rust D3DBLEND_SRCALPHASAT        = d3d::Blend::SrcAlphaSat
//#cpp2rust D3DBLEND_BOTHSRCALPHA       = d3d::Blend::BothSrcAlpha
//#cpp2rust D3DBLEND_BOTHINVSRCALPHA    = d3d::Blend::BothInvSrcAlpha
//#cpp2rust D3DBLEND_BLENDFACTOR        = d3d::Blend::BlendFactor
//#cpp2rust D3DBLEND_INVBLENDFACTOR     = d3d::Blend::InvBlendFactor
//#cpp2rust D3DBLEND_SRCCOLOR2          = d3d::Blend::SrcColor2
//#cpp2rust D3DBLEND_INVSRCCOLOR2       = d3d::Blend::InvSrcColor2
