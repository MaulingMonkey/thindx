#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureaddress)\]
/// D3DTEXTUREADDRESS
///
/// Describes the supported texture-addressing modes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct TextureAddress(D3DTEXTUREADDRESS);
pub use TextureAddress as TAddress;

enumish! { TAddress => D3DTEXTUREADDRESS; Wrap, Mirror, Clamp, Border, MirrorOnce }

#[allow(non_upper_case_globals)] impl TextureAddress { // These are enum-like
    pub const Wrap          : TextureAddress = TextureAddress(D3DTADDRESS_WRAP);
    pub const Mirror        : TextureAddress = TextureAddress(D3DTADDRESS_MIRROR);
    pub const Clamp         : TextureAddress = TextureAddress(D3DTADDRESS_CLAMP);
    pub const Border        : TextureAddress = TextureAddress(D3DTADDRESS_BORDER);
    pub const MirrorOnce    : TextureAddress = TextureAddress(D3DTADDRESS_MIRRORONCE);
}

impl Default for TextureAddress {
    fn default() -> Self { TextureAddress::Wrap } // 1
}

//#cpp2rust D3DTEXTUREADDRESS       = d3d::TextureAddress
//#cpp2rust D3DTADDRESS_WRAP        = d3d::TAddress::Wrap
//#cpp2rust D3DTADDRESS_MIRROR      = d3d::TAddress::Mirror
//#cpp2rust D3DTADDRESS_CLAMP       = d3d::TAddress::Clamp
//#cpp2rust D3DTADDRESS_BORDER      = d3d::TAddress::Border
//#cpp2rust D3DTADDRESS_MIRRORONCE  = d3d::TAddress::MirrorOnce
