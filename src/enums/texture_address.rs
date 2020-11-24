#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureaddress)\]
/// D3DTEXTUREADDRESS
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TextureAddress(D3DTEXTUREADDRESS);
pub type TAddress = TextureAddress;

impl TextureAddress {
    /// Convert a raw [D3DTEXTUREADDRESS] value into a [TextureAddress].  This is *probably* safe... probably....
    ///
    /// [D3DTEXTUREADDRESS]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureaddress
    pub const fn from_unchecked(textureaddress: D3DTEXTUREADDRESS) -> Self { Self(textureaddress) }

    /// Convert a [TextureAddress] into a raw [D3DTEXTUREADDRESS].
    ///
    /// [D3DTEXTUREADDRESS]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureaddress
    pub const fn into(self) -> D3DTEXTUREADDRESS { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl TextureAddress {
    pub const Wrap          : TextureAddress = TextureAddress(D3DTADDRESS_WRAP);
    pub const Mirror        : TextureAddress = TextureAddress(D3DTADDRESS_MIRROR);
    pub const Clamp         : TextureAddress = TextureAddress(D3DTADDRESS_CLAMP);
    pub const Border        : TextureAddress = TextureAddress(D3DTADDRESS_BORDER);
    pub const MirrorOnce    : TextureAddress = TextureAddress(D3DTADDRESS_MIRRORONCE);
}

impl Default for TextureAddress {
    fn default() -> Self { TextureAddress::Wrap }
}

impl Debug for TextureAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TextureAddress::Wrap        => write!(f, "TextureAddress::Wrap"),
            TextureAddress::Mirror      => write!(f, "TextureAddress::Mirror"),
            TextureAddress::Clamp       => write!(f, "TextureAddress::Clamp"),
            TextureAddress::Border      => write!(f, "TextureAddress::Border"),
            TextureAddress::MirrorOnce  => write!(f, "TextureAddress::MirrorOnce"),
            other                       => write!(f, "TextureAddress({})", other.0),
        }
    }
}

impl From<TextureAddress> for D3DTEXTUREADDRESS {
    fn from(value: TextureAddress) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DTEXTUREADDRESS> for TextureAddress {
    fn from(value: D3DTEXTUREADDRESS) -> Self { Self(value) }
}
