#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblend)\]
/// D3DBLEND
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Blend(D3DBLEND);

impl Blend {
    /// Convert a raw [D3DBLEND] value into a [Blend].  This is *probably* safe... probably....
    ///
    /// [D3DBLEND]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblend
    pub const fn from_unchecked(blend: D3DBLEND) -> Self { Self(blend) }

    /// Convert a [Blend] into a raw [D3DBLEND].
    ///
    /// [D3DBLEND]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblend
    pub const fn into(self) -> D3DBLEND { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Blend {
    pub const Zero                  : Blend = Blend(D3DBLEND_ZERO); // 1 - ironic, I know
    pub const One                   : Blend = Blend(D3DBLEND_ONE);  // 2 - herp derp
    pub const SrcColor              : Blend = Blend(D3DBLEND_SRCCOLOR);
    pub const InvSrcColor           : Blend = Blend(D3DBLEND_INVSRCCOLOR);
    pub const Srcalpha              : Blend = Blend(D3DBLEND_SRCALPHA);
    pub const InvSrcAlpha           : Blend = Blend(D3DBLEND_INVSRCALPHA);
    pub const DestAlpha             : Blend = Blend(D3DBLEND_DESTALPHA);
    pub const InvDestAlpha          : Blend = Blend(D3DBLEND_INVDESTALPHA);
    pub const DestColor             : Blend = Blend(D3DBLEND_DESTCOLOR);
    pub const InvDestColor          : Blend = Blend(D3DBLEND_INVDESTCOLOR);
    pub const SrcAlphaSat           : Blend = Blend(D3DBLEND_SRCALPHASAT);
    pub const BothSrcalpha          : Blend = Blend(D3DBLEND_BOTHSRCALPHA);
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

impl Debug for Blend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Blend::Zero             => write!(f, "Blend::Zero"),
            Blend::One              => write!(f, "Blend::One"),
            Blend::SrcColor         => write!(f, "Blend::SrcColor"),
            Blend::InvSrcColor      => write!(f, "Blend::InvSrcColor"),
            Blend::Srcalpha         => write!(f, "Blend::Srcalpha"),
            Blend::InvSrcAlpha      => write!(f, "Blend::InvSrcAlpha"),
            Blend::DestAlpha        => write!(f, "Blend::DestAlpha"),
            Blend::InvDestAlpha     => write!(f, "Blend::InvDestAlpha"),
            Blend::DestColor        => write!(f, "Blend::DestColor"),
            Blend::InvDestColor     => write!(f, "Blend::InvDestColor"),
            Blend::SrcAlphaSat      => write!(f, "Blend::SrcAlphaSat"),
            Blend::BothSrcalpha     => write!(f, "Blend::BothSrcalpha"),
            Blend::BothInvSrcAlpha  => write!(f, "Blend::BothInvSrcAlpha"),
            Blend::BlendFactor      => write!(f, "Blend::BlendFactor"),
            Blend::InvBlendFactor   => write!(f, "Blend::InvBlendFactor"),
            Blend::SrcColor2        => write!(f, "Blend::SrcColor2"),
            Blend::InvSrcColor2     => write!(f, "Blend::InvSrcColor2"),
            other                   => write!(f, "Blend({})", other.0),
        }
    }
}

impl From<Blend> for D3DBLEND {
    fn from(value: Blend) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DBLEND> for Blend {
    fn from(value: D3DBLEND) -> Self { Self(value) }
}
