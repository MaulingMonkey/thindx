#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureaddress)\]
/// D3DTEXTUREADDRESS
///
/// Describes the supported texture-addressing modes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TextureAddress(D3DTEXTUREADDRESS);
pub type TAddress = TextureAddress;

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
