#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dformat)\] D3DFORMAT
///
/// Enumeration cases are labeled in native endian.
///
/// <h3>RGBA Color Formats</h3>
///
/// | Format                    | Bits/Pixel  | Back Buffer | Display | Native Endian Bits  |
/// | ------------------------- | ----------- | ----------- | ------- | ------------------- |
/// | [`Format::UNKNOWN`]       | <span style="opacity: 25%">N/A</span> | <span style="opacity: 25%">N/A</span> | <span style="opacity: 25%">N/A</span> | <span style="opacity: 25%">N/A</span>
/// | [`Format::R8G8B8`]        | 24    |   |   | <code style="font-weight: bold"><span style="color: red">rrrrrrrr</span><span style="color: green">gggggggg</span><span style="color: blue">bbbbbbbb</span></code>
/// | [`Format::A8R8G8B8`]      | 32    |✔️| ❌| <code style="font-weight: bold"><span style="opacity: 100%">aaaaaaaa</span><span style="color: red">rrrrrrrr</span><span style="color: green">gggggggg</span><span style="color: blue">bbbbbbbb</span></code>
/// | [`Format::X8R8G8B8`]      | 32    |✔️| ✔️| <code style="font-weight: bold"><span style="opacity:  33%">xxxxxxxx</span><span style="color: red">rrrrrrrr</span><span style="color: green">gggggggg</span><span style="color: blue">bbbbbbbb</span></code>
/// | [`Format::R5G6B5`]        | 16    |✔️| ✔️| <code style="font-weight: bold"><span style="color: red">rrrrr</span><span style="color: green">gggggg</span><span style="color: blue">bbbbb</span></code>
/// | [`Format::X1R5G5B5`]      | 16    |✔️| ✔️| <code style="font-weight: bold"><span style="opacity:  33%">x</span><span style="color: red">rrrrr</span><span style="color: green">ggggg</span><span style="color: blue">bbbbb</span></code>
/// | [`Format::A1R5G5B5`]      | 16    |✔️| ❌| <code style="font-weight: bold"><span style="opacity: 100%">a</span><span style="color: red">rrrrr</span><span style="color: green">ggggg</span><span style="color: blue">bbbbb</span></code>
/// | [`Format::A4R4G4B4`]      | 16    |   |   | <code style="font-weight: bold"><span style="opacity: 100%">aaaa</span><span style="color: red">rrrr</span><span style="color: green">gggg</span><span style="color: blue">bbbb</span></code>
/// | [`Format::R3G3B2`]        | 8     |   |   | <code style="font-weight: bold"><span style="color: red">rrr</span><span style="color: green">ggg</span><span style="color: blue">bb</span></code>
/// | [`Format::A8`]            | 8     |   |   | <code style="font-weight: bold"><span style="opacity: 100%">aaaaaaaa</span></code>
/// | [`Format::A8R3G3B2`]      | 16    |   |   | <code style="font-weight: bold"><span style="opacity: 100%">aaaaaaaa</span><span style="color: red">rrr</span><span style="color: green">ggg</span><span style="color: blue">bb</span></code>
/// | [`Format::X4R4G4B4`]      | 16    |   |   | <code style="font-weight: bold"><span style="opacity:  33%">xxxx</span><span style="color: red">rrrr</span><span style="color: green">gggg</span><span style="color: blue">bbbb</span></code>
/// | [`Format::A2B10G10R10`]   | 32    |   |   | <code style="font-weight: bold"><span style="opacity: 100%">aa</span><span style="color: blue">bbbbbbbbbb</span><span style="color: green">gggggggggg</span><span style="color: red">rrrrrrrrrr</span></code>
/// | [`Format::A8B8G8R8`]      | 32    |   |   | <code style="font-weight: bold"><span style="opacity: 100%">aaaaaaaa</span><span style="color: blue">bbbbbbbb</span><span style="color: green">gggggggg</span><span style="color: red">rrrrrrrr</span></code>
/// | [`Format::X8B8G8R8`]      | 32    |   |   | <code style="font-weight: bold"><span style="opacity:  33%">xxxxxxxx</span><span style="color: blue">bbbbbbbb</span><span style="color: green">gggggggg</span><span style="color: red">rrrrrrrr</span></code>
/// | [`Format::G16R16`]        | 32    |   |   | <code style="font-weight: bold"><span style="color: green">gggggggggggggggg</span><span style="color: red">rrrrrrrrrrrrrrrr</span></code>
/// | [`Format::A2R10G10B10`]   | 32    | ✔️ | ✔️ | <code style="font-weight: bold"><span style="opacity: 100%">aa</span><span style="color: red">rrrrrrrrrr</span><span style="color: green">gggggggggg</span><span style="color: blue">bbbbbbbbbb</span></code>
/// | [`Format::A16B16G16R16`]  | 64    |   |   | <code style="font-weight: bold"><span style="opacity: 100%">AAAA</span><span style="color: red">RRRR</span><span style="color: green">GGGG</span><span style="color: blue">BBBB</span></code><sub>16</sub>
/// | **[Direct3DEx] Only**     |
/// | [`Format::A1`]                   | 1  | | | <code style="font-weight: bold; opacity: 100%">a</code>
/// | [`Format::A2B10G10R10_XR_BIAS`]  | 32 | | | ???
///
/// | Key                                                           | Description |
/// | ------------------------------------------------------------- | ----------- |
/// | <code style="font-weight: bold; color: red">rrrr</code>       | Red channel bits
/// | <code style="font-weight: bold; color: green">gggg</code>     | Green channel bits
/// | <code style="font-weight: bold; color: blue">bbbb</code>      | Blue channel bits
/// | <code style="font-weight: bold; opacity: 100%">aaaa</code>    | Alpha channel bits
/// | <code style="font-weight: bold; opacity:  33%">xxxx</code>    | Unused channel bits
///
/// <h3>Non-RGBA Color Formats</h3>
///
/// | Format                    | Bits/Pixel | Native Endian Bits   |
/// | ------------------------- | ----- | ------------------------- |
/// | [`Format::A8P8`]          | 16    | <code style="font-weight: bold"><span style="opacity: 100%">aaaaaaaa</span><span style="color: purple">pppppppp</span></code>
/// | [`Format::P8`]            | 8     | <code style="font-weight: bold; color: purple">pppppppp</code>
/// | [`Format::L8`]            | 8     | <code style="font-weight: bold; color: orange">llllllll</code>
/// | [`Format::A8L8`]          | 16    | <code style="font-weight: bold"><span style="opacity: 100%">aaaaaaaa</span><span style="color: orange">llllllll</span></code>
/// | [`Format::A4L4`]          | 8     | <code style="font-weight: bold"><span style="opacity: 100%">aaaa</span><span style="color: orange">llll</span></code>
/// | [`Format::L16`]           | 16    | <code style="font-weight: bold; color: orange">llllllllllllllll</code>
///
/// | Key                                                           | Description |
/// | ------------------------------------------------------------- | ----------- |
/// | <code style="font-weight: bold; opacity: 100%">aaaa</code>    | Alpha channel bits
/// | <code style="font-weight: bold; color: purple">pppp</code>    | Pallete index bits
/// | <code style="font-weight: bold; color: orange">llll</code>    | Luminance bits (red+green+blue?)
///
/// <h3>Compressed Color Formats</h3>
///
/// | [X3TC] Formats            | Bits/Block | Block  | BC# | Alpha |
/// | ------------------------- | ---- | ------ | --- | ----- |
/// | [`Format::DXT1`]          |  64  | 4x4    | BC1 | 1-bit
/// | [`Format::DXT2`]          | 128  | 4x4    | BC2 | Sharp, premul
/// | [`Format::DXT3`]          | 128  | 4x4    | BC2 | Sharp, non-premul
/// | [`Format::DXT4`]          | 128  | 4x4    | BC3 | Gradient, premul
/// | [`Format::DXT5`]          | 128  | 4x4    | BC3 | Gradient, non-premul
///
/// [X3TC]:             https://en.wikipedia.org/wiki/X3_Texture_Compression
///
/// <h3>Depth/Stencil Formats</h3>
///
/// | Format                    | Bits/Pixel    | Native Endian Bits |
/// | ------------------------- | ------------- | ------------------ |
/// | [`Format::D16_LOCKABLE`]  | 16            | <code style="font-weight: bold">dddddddddddddddd</code>
/// | [`Format::D32`]           | 32            | <code style="font-weight: bold">dddddddddddddddddddddddddddddddd</code>
/// | [`Format::D15S1`]         | 16            | <code style="font-weight: bold">ddddddddddddddd<span style="color: red">s</span></code>
/// | [`Format::D24S8`]         | 32            | <code style="font-weight: bold">dddddddddddddddddddddddd<span style="color: red">ssssssss</span></code>
/// | [`Format::D24X8`]         | 32            | <code style="font-weight: bold">dddddddddddddddddddddddd<span style="opacity: 33%">xxxxxxxx</span></code>
/// | [`Format::D24X4S4`]       | 32            | <code style="font-weight: bold">dddddddddddddddddddddddd<span style="opacity: 33%">xxxx</span><span style="color: red">ssss</span></code>
/// | [`Format::D16`]           | 16            | <code style="font-weight: bold">dddddddddddddddd</code>
/// | [`Format::D32F_LOCKABLE`] | 32            | <code style="font-weight: bold">dddddddddddddddddddddddddddddddd</code>
/// | [`Format::D24FS8`]        | 32            | <code style="font-weight: bold">dddddddddddddddddddddddd<span style="color: red">ssssssss</span></code>
/// | **[Direct3DEx] Only**     |
/// | [`Format::D32_LOCKABLE`]  | 32            | <code style="font-weight: bold">dddddddddddddddddddddddddddddddd</code>
/// | [`Format::X8_LOCKABLE`]   | 8             | <code style="font-weight: bold; color: red">ssssssss</code>
///
/// <h3>Data Formats</h3>
///
/// | Format                    | Bits/Element  |
/// | ------------------------- | ------------- |
/// | [`Format::BINARYBUFFER`]  | <span style="opacity: 25%">N/A</span> |
/// | [`Format::VERTEXDATA`]    | <span style="opacity: 25%">N/A</span> |
/// | [`Format::INDEX16`]       | 16            |
/// | [`Format::INDEX32`]       | 32            |
///
/// <h3>Uncategorized Formats</h3>
///
/// | Format                    |
/// | ------------------------- |
/// | [`Format::UYVY`]          |
/// | [`Format::R8G8_B8G8`]     |
/// | [`Format::YUY2`]          |
/// | [`Format::G8R8_G8B8`]     |
/// | [`Format::V8U8`]          |
/// | |
/// | [`Format::L6V5U5`]        |
/// | [`Format::X8L8V8U8`]      |
/// | [`Format::Q8W8V8U8`]      |
/// | [`Format::V16U16`]        |
/// | [`Format::A2W10V10U10`]   |
/// | |
/// | [`Format::Q16W16V16U16`]  |
/// | [`Format::MULTI2_ARGB8`]  |
/// | |
/// | [`Format::R16F`]          |
/// | [`Format::G16R16F`]       |
/// | [`Format::A16B16G16R16F`] |
/// | [`Format::R32F`]          |
/// | [`Format::G32R32F`]       |
/// | [`Format::A32B32G32R32F`] |
/// | |
/// | [`Format::CxV8U8`]        |
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Format(u32);
pub use Format as Fmt;

impl Format {
    pub const fn zeroed() -> Self { Self(0) }

    /// Convert a raw [D3DFORMAT] value into a [Format].  This is *probably* safe... probably...
    ///
    /// [D3DFORMAT]:     https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dformat
    pub const fn from_unchecked(format: D3DFORMAT) -> Self { Self(format) }

    /// Convert a [Format] back into a [D3DFORMAT].
    ///
    /// [D3DFORMAT]:     https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dformat
    pub const fn into(self) -> D3DFORMAT { self.0 }

}

#[allow(non_upper_case_globals)] // enum-like
impl Format {
    /// Surface format is unknown
    pub const UNKNOWN               : Format = Format(D3DFMT_UNKNOWN);
    pub const Unknown               : Format = Format::UNKNOWN;

    pub const R8G8B8                : Format = Format(D3DFMT_R8G8B8);
    pub const A8R8G8B8              : Format = Format(D3DFMT_A8R8G8B8);
    pub const X8R8G8B8              : Format = Format(D3DFMT_X8R8G8B8);
    pub const R5G6B5                : Format = Format(D3DFMT_R5G6B5);
    pub const X1R5G5B5              : Format = Format(D3DFMT_X1R5G5B5);
    pub const A1R5G5B5              : Format = Format(D3DFMT_A1R5G5B5);
    pub const A4R4G4B4              : Format = Format(D3DFMT_A4R4G4B4);
    pub const R3G3B2                : Format = Format(D3DFMT_R3G3B2);
    pub const A8                    : Format = Format(D3DFMT_A8);
    pub const A8R3G3B2              : Format = Format(D3DFMT_A8R3G3B2);
    pub const X4R4G4B4              : Format = Format(D3DFMT_X4R4G4B4);
    pub const A2B10G10R10           : Format = Format(D3DFMT_A2B10G10R10);
    pub const A8B8G8R8              : Format = Format(D3DFMT_A8B8G8R8);
    pub const X8B8G8R8              : Format = Format(D3DFMT_X8B8G8R8);
    pub const G16R16                : Format = Format(D3DFMT_G16R16);
    pub const A2R10G10B10           : Format = Format(D3DFMT_A2R10G10B10);
    pub const A16B16G16R16          : Format = Format(D3DFMT_A16B16G16R16);

    pub const A8P8                  : Format = Format(D3DFMT_A8P8);
    pub const P8                    : Format = Format(D3DFMT_P8);

    pub const L8                    : Format = Format(D3DFMT_L8);
    pub const A8L8                  : Format = Format(D3DFMT_A8L8);
    pub const A4L4                  : Format = Format(D3DFMT_A4L4);

    // table up to here

    pub const V8U8                  : Format = Format(D3DFMT_V8U8);
    pub const L6V5U5                : Format = Format(D3DFMT_L6V5U5);
    pub const X8L8V8U8              : Format = Format(D3DFMT_X8L8V8U8);
    pub const Q8W8V8U8              : Format = Format(D3DFMT_Q8W8V8U8);
    pub const V16U16                : Format = Format(D3DFMT_V16U16);
    pub const A2W10V10U10           : Format = Format(D3DFMT_A2W10V10U10);

    pub const UYVY                  : Format = Format(D3DFMT_UYVY);
    pub const R8G8_B8G8             : Format = Format(D3DFMT_R8G8_B8G8);
    pub const YUY2                  : Format = Format(D3DFMT_YUY2);
    pub const G8R8_G8B8             : Format = Format(D3DFMT_G8R8_G8B8);
    pub const DXT1                  : Format = Format(D3DFMT_DXT1);
    pub const DXT2                  : Format = Format(D3DFMT_DXT2);
    pub const DXT3                  : Format = Format(D3DFMT_DXT3);
    pub const DXT4                  : Format = Format(D3DFMT_DXT4);
    pub const DXT5                  : Format = Format(D3DFMT_DXT5);

    pub const D16_LOCKABLE          : Format = Format(D3DFMT_D16_LOCKABLE);
    pub const D32                   : Format = Format(D3DFMT_D32);
    pub const D15S1                 : Format = Format(D3DFMT_D15S1);
    pub const D24S8                 : Format = Format(D3DFMT_D24S8);
    pub const D24X8                 : Format = Format(D3DFMT_D24X8);
    pub const D24X4S4               : Format = Format(D3DFMT_D24X4S4);
    pub const D16                   : Format = Format(D3DFMT_D16);

    pub const D32F_LOCKABLE         : Format = Format(D3DFMT_D32F_LOCKABLE);
    pub const D24FS8                : Format = Format(D3DFMT_D24FS8);

    /// Direct3D 9Ex Only
    pub const D32_LOCKABLE          : Format = Format(D3DFMT_D32_LOCKABLE);

    /// Direct3D 9Ex Only
    pub const X8_LOCKABLE           : Format = Format(D3DFMT_S8_LOCKABLE);

    pub const L16                   : Format = Format(D3DFMT_L16);

    pub const VERTEXDATA            : Format = Format(D3DFMT_VERTEXDATA);
    pub const INDEX16               : Format = Format(D3DFMT_INDEX16);
    pub const INDEX32               : Format = Format(D3DFMT_INDEX32);
    pub const VertexData            : Format = Format::VERTEXDATA;
    pub const Index16               : Format = Format::INDEX16;
    pub const Index32               : Format = Format::INDEX32;

    pub const Q16W16V16U16          : Format = Format(D3DFMT_Q16W16V16U16);

    /// See [Multiple-element Textures](https://docs.microsoft.com/en-us/windows/win32/direct3d9/multiple-element-textures)
    pub const MULTI2_ARGB8          : Format = Format(D3DFMT_MULTI2_ARGB8);

    pub const R16F                  : Format = Format(D3DFMT_R16F);
    pub const G16R16F               : Format = Format(D3DFMT_G16R16F);
    pub const A16B16G16R16F         : Format = Format(D3DFMT_A16B16G16R16F);

    pub const R32F                  : Format = Format(D3DFMT_R32F);
    pub const G32R32F               : Format = Format(D3DFMT_G32R32F);
    pub const A32B32G32R32F         : Format = Format(D3DFMT_A32B32G32R32F);

    #[allow(non_upper_case_globals)]
    pub const CxV8U8                : Format = Format(D3DFMT_CxV8U8);

    /// 1-bit monochrome.
    ///
    /// Direct3D 9Ex Only
    pub const A1                    : Format = Format(D3DFMT_A1);

    /// 2.8-biased fixed point.
    ///
    /// Direct3D 9Ex Only
    pub const A2B10G10R10_XR_BIAS   : Format = Format(D3DFMT_A2B10G10R10_XR_BIAS);

    /// Binary format indicating that the data has no inherent type.
    ///
    /// Direct3D 9Ex Only
    pub const BINARYBUFFER          : Format = Format(D3DFMT_BINARYBUFFER);

    /// Binary format indicating that the data has no inherent type.
    ///
    /// Direct3D 9Ex Only
    pub const BinaryBuffer          : Format = Format::BINARYBUFFER;
}

impl Format {
    fn d3dfmt(&self) -> &'static str {
        match *self {
            Format::UNKNOWN                                     => "D3DFMT_UNKNOWN",
            Format::R8G8B8                                      => "D3DFMT_R8G8B8",
            Format::A8R8G8B8                                    => "D3DFMT_A8R8G8B8",
            Format::X8R8G8B8                                    => "D3DFMT_X8R8G8B8",
            Format::R5G6B5                                      => "D3DFMT_R5G6B5",
            Format::X1R5G5B5                                    => "D3DFMT_X1R5G5B5",
            Format::A1R5G5B5                                    => "D3DFMT_A1R5G5B5",
            Format::A4R4G4B4                                    => "D3DFMT_A4R4G4B4",
            Format::R3G3B2                                      => "D3DFMT_R3G3B2",
            Format::A8                                          => "D3DFMT_A8",
            Format::A8R3G3B2                                    => "D3DFMT_A8R3G3B2",
            Format::X4R4G4B4                                    => "D3DFMT_X4R4G4B4",
            Format::A2B10G10R10                                 => "D3DFMT_A2B10G10R10",
            Format::A8B8G8R8                                    => "D3DFMT_A8B8G8R8",
            Format::X8B8G8R8                                    => "D3DFMT_X8B8G8R8",
            Format::G16R16                                      => "D3DFMT_G16R16",
            Format::A2R10G10B10                                 => "D3DFMT_A2R10G10B10",
            Format::A16B16G16R16                                => "D3DFMT_A16B16G16R16",
            Format::A8P8                                        => "D3DFMT_A8P8",
            Format::P8                                          => "D3DFMT_P8",
            Format::L8                                          => "D3DFMT_L8",
            Format::A8L8                                        => "D3DFMT_A8L8",
            Format::A4L4                                        => "D3DFMT_A4L4",

            Format::V8U8                                        => "D3DFMT_V8U8",
            Format::L6V5U5                                      => "D3DFMT_L6V5U5",
            Format::X8L8V8U8                                    => "D3DFMT_X8L8V8U8",
            Format::Q8W8V8U8                                    => "D3DFMT_Q8W8V8U8",
            Format::V16U16                                      => "D3DFMT_V16U16",
            Format::A2W10V10U10                                 => "D3DFMT_A2W10V10U10",
            Format::UYVY                                        => "D3DFMT_UYVY",
            Format::R8G8_B8G8                                   => "D3DFMT_R8G8_B8G8",
            Format::YUY2                                        => "D3DFMT_YUY2",
            Format::G8R8_G8B8                                   => "D3DFMT_G8R8_G8B8",
            Format::DXT1                                        => "D3DFMT_DXT1",
            Format::DXT2                                        => "D3DFMT_DXT2",
            Format::DXT3                                        => "D3DFMT_DXT3",
            Format::DXT4                                        => "D3DFMT_DXT4",
            Format::DXT5                                        => "D3DFMT_DXT5",
            Format::D16_LOCKABLE                                => "D3DFMT_D16_LOCKABLE",
            Format::D32                                         => "D3DFMT_D32",
            Format::D15S1                                       => "D3DFMT_D15S1",
            Format::D24S8                                       => "D3DFMT_D24S8",
            Format::D24X8                                       => "D3DFMT_D24X8",
            Format::D24X4S4                                     => "D3DFMT_D24X4S4",
            Format::D16                                         => "D3DFMT_D16",
            Format::D32F_LOCKABLE                               => "D3DFMT_D32F_LOCKABLE",
            Format::D24FS8                                      => "D3DFMT_D24FS8",

            Format::D32_LOCKABLE                                => "D3DFMT_D32_LOCKABLE",
            Format::X8_LOCKABLE                                 => "D3DFMT_S8_LOCKABLE",

            Format::L16                                         => "D3DFMT_L16",
            Format::VERTEXDATA                                  => "D3DFMT_VERTEXDATA",
            Format::INDEX16                                     => "D3DFMT_INDEX16",
            Format::INDEX32                                     => "D3DFMT_INDEX32",
            Format::Q16W16V16U16                                => "D3DFMT_Q16W16V16U16",
            Format::MULTI2_ARGB8                                => "D3DFMT_MULTI2_ARGB8",
            Format::R16F                                        => "D3DFMT_R16F",
            Format::G16R16F                                     => "D3DFMT_G16R16F",
            Format::A16B16G16R16F                               => "D3DFMT_A16B16G16R16F",
            Format::R32F                                        => "D3DFMT_R32F",
            Format::G32R32F                                     => "D3DFMT_G32R32F",
            Format::A32B32G32R32F                               => "D3DFMT_A32B32G32R32F",
            Format::CxV8U8                                      => "D3DFMT_CxV8U8",

            Format::A1                                          => "D3DFMT_A1",
            Format::A2B10G10R10_XR_BIAS                         => "D3DFMT_A2B10G10R10_XR_BIAS",
            Format::BINARYBUFFER                                => "D3DFMT_BINARYBUFFER",

            _other                                              => "D3DFMT_???",
        }
    }
}

impl Debug for Format {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let fmt = self.d3dfmt();
        write!(f, "Format({}", fmt)?;
        if fmt == "D3DFMT_???" { write!(f, " (0x{:08x})", self.0 as u32)?; }
        write!(f, ")")
    }
}

impl From<Format> for D3DFORMAT {
    fn from(value: Format) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DFORMAT> for Format {
    fn from(value: D3DFORMAT) -> Self { Self(value) }
}

//#cpp2rust D3DFORMAT                   = d3d::Format
//#cpp2rust D3DFMT_UNKNOWN              = d3d::Format::UNKNOWN
//#cpp2rust D3DFMT_R8G8B8               = d3d::Format::R8G8B8
//#cpp2rust D3DFMT_A8R8G8B8             = d3d::Format::A8R8G8B8
//#cpp2rust D3DFMT_X8R8G8B8             = d3d::Format::X8R8G8B8
//#cpp2rust D3DFMT_R5G6B5               = d3d::Format::R5G6B5
//#cpp2rust D3DFMT_X1R5G5B5             = d3d::Format::X1R5G5B5
//#cpp2rust D3DFMT_A1R5G5B5             = d3d::Format::A1R5G5B5
//#cpp2rust D3DFMT_A4R4G4B4             = d3d::Format::A4R4G4B4
//#cpp2rust D3DFMT_R3G3B2               = d3d::Format::R3G3B2
//#cpp2rust D3DFMT_A8                   = d3d::Format::A8
//#cpp2rust D3DFMT_A8R3G3B2             = d3d::Format::A8R3G3B2
//#cpp2rust D3DFMT_X4R4G4B4             = d3d::Format::X4R4G4B4
//#cpp2rust D3DFMT_A2B10G10R10          = d3d::Format::A2B10G10R10
//#cpp2rust D3DFMT_A8B8G8R8             = d3d::Format::A8B8G8R8
//#cpp2rust D3DFMT_X8B8G8R8             = d3d::Format::X8B8G8R8
//#cpp2rust D3DFMT_G16R16               = d3d::Format::G16R16
//#cpp2rust D3DFMT_A2R10G10B10          = d3d::Format::A2R10G10B10
//#cpp2rust D3DFMT_A16B16G16R16         = d3d::Format::A16B16G16R16
//#cpp2rust D3DFMT_A8P8                 = d3d::Format::A8P8
//#cpp2rust D3DFMT_P8                   = d3d::Format::P8
//#cpp2rust D3DFMT_L8                   = d3d::Format::L8
//#cpp2rust D3DFMT_A8L8                 = d3d::Format::A8L8
//#cpp2rust D3DFMT_A4L4                 = d3d::Format::A4L4
//#cpp2rust D3DFMT_V8U8                 = d3d::Format::V8U8
//#cpp2rust D3DFMT_L6V5U5               = d3d::Format::L6V5U5
//#cpp2rust D3DFMT_X8L8V8U8             = d3d::Format::X8L8V8U8
//#cpp2rust D3DFMT_Q8W8V8U8             = d3d::Format::Q8W8V8U8
//#cpp2rust D3DFMT_V16U16               = d3d::Format::V16U16
//#cpp2rust D3DFMT_A2W10V10U10          = d3d::Format::A2W10V10U10
//#cpp2rust D3DFMT_UYVY                 = d3d::Format::UYVY
//#cpp2rust D3DFMT_R8G8_B8G8            = d3d::Format::R8G8_B8G8
//#cpp2rust D3DFMT_YUY2                 = d3d::Format::YUY2
//#cpp2rust D3DFMT_G8R8_G8B8            = d3d::Format::G8R8_G8B8
//#cpp2rust D3DFMT_DXT1                 = d3d::Format::DXT1
//#cpp2rust D3DFMT_DXT2                 = d3d::Format::DXT2
//#cpp2rust D3DFMT_DXT3                 = d3d::Format::DXT3
//#cpp2rust D3DFMT_DXT4                 = d3d::Format::DXT4
//#cpp2rust D3DFMT_DXT5                 = d3d::Format::DXT5
//#cpp2rust D3DFMT_D16_LOCKABLE         = d3d::Format::D16_LOCKABLE
//#cpp2rust D3DFMT_D32                  = d3d::Format::D32
//#cpp2rust D3DFMT_D15S1                = d3d::Format::D15S1
//#cpp2rust D3DFMT_D24S8                = d3d::Format::D24S8
//#cpp2rust D3DFMT_D24X8                = d3d::Format::D24X8
//#cpp2rust D3DFMT_D24X4S4              = d3d::Format::D24X4S4
//#cpp2rust D3DFMT_D16                  = d3d::Format::D16
//#cpp2rust D3DFMT_D32F_LOCKABLE        = d3d::Format::D32F_LOCKABLE
//#cpp2rust D3DFMT_D24FS8               = d3d::Format::D24FS8
//#cpp2rust D3DFMT_D32_LOCKABLE         = d3d::Format::D32_LOCKABLE
//#cpp2rust D3DFMT_S8_LOCKABLE          = d3d::Format::X8_LOCKABLE
//#cpp2rust D3DFMT_L16                  = d3d::Format::L16
//#cpp2rust D3DFMT_VERTEXDATA           = d3d::Format::VERTEXDATA
//#cpp2rust D3DFMT_INDEX16              = d3d::Format::INDEX16
//#cpp2rust D3DFMT_INDEX32              = d3d::Format::INDEX32
//#cpp2rust D3DFMT_Q16W16V16U16         = d3d::Format::Q16W16V16U16
//#cpp2rust D3DFMT_MULTI2_ARGB8         = d3d::Format::MULTI2_ARGB8
//#cpp2rust D3DFMT_R16F                 = d3d::Format::R16F
//#cpp2rust D3DFMT_G16R16F              = d3d::Format::G16R16F
//#cpp2rust D3DFMT_A16B16G16R16F        = d3d::Format::A16B16G16R16F
//#cpp2rust D3DFMT_R32F                 = d3d::Format::R32F
//#cpp2rust D3DFMT_G32R32F              = d3d::Format::G32R32F
//#cpp2rust D3DFMT_A32B32G32R32F        = d3d::Format::A32B32G32R32F
//#cpp2rust D3DFMT_CxV8U8               = d3d::Format::CxV8U8
//#cpp2rust D3DFMT_A1                   = d3d::Format::A1
//#cpp2rust D3DFMT_A2B10G10R10_XR_BIAS  = d3d::Format::A2B10G10R10_XR_BIAS
//#cpp2rust D3DFMT_BINARYBUFFER         = d3d::Format::BINARYBUFFER
