#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::ops::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcolor)\]
/// D3DCOLOR
///
/// 0xAA<span style="color: red">RR</span><span style="color: green">GG</span><span style="color: blue">BB</span>
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Color(D3DCOLOR);

impl Color {
    /// ### Examples
    ///
    /// ```rust
    /// # use thin3d9::*;
    /// let colors = [
    ///     Color::argb(0xFF224466),                // native endian
    ///     Color::argb([0xFF, 0x22, 0x44, 0x66]),  // big endian!
    ///     Color::argb((0xFF, 0x22, 0x44, 0x66)),  // big endian!
    /// ];
    /// for color in colors.iter().copied() {
    ///     assert_eq!(0xFF, color.alpha());
    ///     assert_eq!(0x22, color.red());
    ///     assert_eq!(0x44, color.green());
    ///     assert_eq!(0x66, color.blue());
    /// }
    /// ```
    pub fn argb(color: impl AsARGB) -> Self { color.as_argb() }

    pub fn a(self) -> u8 { (self.0 >> 24) as u8 }
    pub fn r(self) -> u8 { (self.0 >> 16) as u8 }
    pub fn g(self) -> u8 { (self.0 >>  8) as u8 }
    pub fn b(self) -> u8 { (self.0 >>  0) as u8 }

    pub fn alpha(self) -> u8 { (self.0 >> 24) as u8 }
    pub fn red  (self) -> u8 { (self.0 >> 16) as u8 }
    pub fn green(self) -> u8 { (self.0 >>  8) as u8 }
    pub fn blue (self) -> u8 { (self.0 >>  0) as u8 }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Color(0x{:08X})", self.0)
    }
}

impl Deref for Color {
    type Target = D3DCOLOR;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DCOLOR> for Color {
    fn from(value: D3DCOLOR) -> Self { Self(value) }
}

impl From<Color> for D3DCOLOR {
    fn from(value: Color) -> Self { value.0 }
}



pub trait AsARGB                    { fn as_argb(self) -> Color; }
impl AsARGB for u32                 { fn as_argb(self) -> Color { Color(self) } }
impl AsARGB for [u8; 4]             { fn as_argb(self) -> Color { Color(u32::from_be_bytes(self)) } }
impl AsARGB for (u8, u8, u8, u8)    { fn as_argb(self) -> Color { let (a,r,g,b) = self; Color(u32::from_be_bytes([a,r,g,b])) } }
