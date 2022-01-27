use crate::*;



/// Similar to [d3d::Format], but limited to texture-friendly formats, and adding metadata to allow safe + sound bounds.
#[derive(Clone, Copy, Debug)]
pub struct FixedTextureFormat(UncheckedTextureFormat);

impl FixedTextureFormat {
    /// Create a [FixedTextureFormat].
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// ThinDX APIs rely on format's fields being accurate/consistent with each other.
    /// Specifying the wrong size or bits per block for a given format may result in buffer
    /// overflows, access of uninitialized data, or other nastiness.
    pub const unsafe fn new(format: UncheckedTextureFormat) -> Self { Self(format) }
}

// DO NOT IMPLEMENT: DerefMut - would break soundness bounds of type.
impl std::ops::Deref for FixedTextureFormat {
    type Target = UncheckedTextureFormat;
    fn deref(&self) -> &Self::Target { &self.0 }
}



/// Similar to [d3d::Format], but presumably limited to texture-friendly formats.
/// Unlike [FixedTextureFormat], there's no guarantee that the values of this type are
#[derive(Clone, Copy, Debug)]
pub struct UncheckedTextureFormat {
    /// The underlying [d3d::Format].
    pub format:         d3d::Format,

    /// Bits per block.
    ///
    /// For most formats, a block synonymous with a single pixel.
    /// However, for e.g. DXT formats, a block is a 4x4 group of pixels.
    pub bits_per_block: u8,

    /// Block size in pixels.
    ///
    /// For most formats, this will be `(1, 1)`.  For DXT formats, this will be `(4, 4)`.
    pub block_size: (u8, u8),
}



macro_rules! formats {
    (unsafe { // we're bypassing FixedTextureFormat::new
        (
            format,
            bits,
            block $(,)?
        )
        $(,
            // TODO: docs/attrs
            (
                $format:ident,
                $bits:literal,
                ($block_width:literal, $block_height:literal) $(,)?
            )
        )*
        $(,)?
    }) => {
        impl FixedTextureFormat {
            $(
                #[allow(missing_docs)] // TODO?
                pub const $format : &'static Self = &Self(UncheckedTextureFormat {
                    format: d3d::Format::$format,
                    bits_per_block: $bits,
                    block_size: ($block_width, $block_height)
                });
            )*

            /// Attempt to create a &[FixedTextureFormat] from a [d3d::Format].
            ///
            /// <h3>Edge Cases of Note</h3>
            ///
            /// *   [d3d::Format::Unknown]      is not supported despite being valid for e.g. back buffer format specifiers
            /// *   [d3d::Format::MULTI2_ARGB8] is not supported due to being variable length depending on device state
            ///
            /// <h3>Not (yet) supported</h3>
            ///
            /// *   [d3d::Format::UYVY]         - TBD
            /// *   [d3d::Format::R8G8_B8G8]    - block size is ambiguous
            /// *   [d3d::Format::YUY2]         - TBD
            /// *   [d3d::Format::G8R8_G8B8]    - block size is ambiguous
            /// *   [d3d::Format::CxV8U8]       - bit count is ambiguous from documentation
            ///
            /// <h3>Never to be supported</h3>
            ///
            /// *   [d3d::Format::VertexData]   - not intended as texture data, variable stride
            /// *   [d3d::Format::Index16]      - not intended as texture data
            /// *   [d3d::Format::Index32]      - not intended as texture data
            /// *   [d3d::Format::BinaryBuffer] - variable stride
            pub fn try_from_standard(format: d3d::Format) -> Option<&'static Self> {
                match format {
                    $( d3d::Format::$format => Some(Self::$format), )*
                    _ => None
                }
            }
        }
    };
}

formats! {
    unsafe {
        (format,              bits, block),

        //(Unknown,             ??, (?,?)),

        (R8G8B8,                24, (1,1)),
        (A8R8G8B8,              32, (1,1)),
        (X8R8G8B8,              32, (1,1)),
        (R5G6B5,                16, (1,1)),
        (X1R5G5B5,              16, (1,1)),
        (A1R5G5B5,              16, (1,1)),
        (A4R4G4B4,              16, (1,1)),
        (R3G3B2,                 8, (1,1)),
        (A8,                     8, (1,1)),
        (A8R3G3B2,              16, (1,1)),
        (X4R4G4B4,              16, (1,1)),
        (A2B10G10R10,           32, (1,1)),
        (A8B8G8R8,              32, (1,1)),
        (X8B8G8R8,              32, (1,1)),
        (G16R16,                32, (1,1)),
        (A2R10G10B10,           32, (1,1)),
        (A16B16G16R16,          64, (1,1)),

        (A8P8,                  16, (1,1)),
        (P8,                     8, (1,1)),

        (L8,                     8, (1,1)),
        (A8L8,                  16, (1,1)),
        (A4L4,                   8, (1,1)),

        (V8U8,                  16, (1,1)),
        (L6V5U5,                16, (1,1)),
        (X8L8V8U8,              32, (1,1)),
        (Q8W8V8U8,              32, (1,1)),
        (V16U16,                32, (1,1)),
        (A2W10V10U10,           32, (1,1)),

        //(UYVY,                ??, (?,?)), // TODO: block size?
        //(R8G8_B8G8,           ??, (?,?)), // TODO: should these be (2,1) block size? or (1,1)?
        //(YUY2,                ??, (?,?)), // TODO: block size?
        //(G8R8_G8B8,           ??, (?,?)), // TODO: should these be (2,1) block size? or (1,1)?
        (DXT1,                  32, (4,4)), // aka BC1 w/  premultiplied alpha
        (DXT2,                  64, (4,4)), // aka BC2 w/  premultiplied alpha
        (DXT3,                  64, (4,4)), // aka BC2 w/o premultiplied alpha
        (DXT4,                  64, (4,4)), // aka BC3 w/  premultiplied alpha
        (DXT5,                  64, (4,4)), // aka BC3 w/o premultiplied alpha

        (D16_LOCKABLE,          16, (1,1)),
        (D32,                   32, (1,1)),
        (D15S1,                 16, (1,1)),
        (D24S8,                 32, (1,1)),
        (D24X8,                 32, (1,1)),
        (D24X4S4,               32, (1,1)),
        (D16,                   16, (1,1)),

        (D32F_LOCKABLE,         32, (1,1)),
        (D24FS8,                32, (1,1)),

        (D32_LOCKABLE,          32, (1,1)),

        (X8_LOCKABLE,            8, (1,1)),

        (L16,                   16, (1,1)),

        //(VertexData,          ??, (?,?)), // not pixel data
        //(Index16,             ??, (?,?)), // not pixel data
        //(Index32,             ??, (?,?)), // not pixel data

        (Q16W16V16U16,          64, (1,1)),

        //(MULTI2_ARGB8,        ??, (?,?)), // variable bits per pixel! https://docs.microsoft.com/en-us/windows/win32/direct3d9/multiple-element-textures

        (R16F,                  16, (1,1)),
        (G16R16F,               32, (1,1)),
        (A16B16G16R16F,         64, (1,1)),

        (R32F,                  32, (1,1)),
        (G32R32F,               64, (1,1)),
        (A32B32G32R32F,        128, (1,1)),

        // I've seen conflicting documentation for if this is 16 or 32 bits.  I assume it's *probably* 16 bits based on the name, but I'm holding off on defining it until this can be confirmed.
        // "A 16-bit normal compression format."    https://docs.microsoft.com/en-us/previous-versions/windows/desktop/bb153349(v=vs.85)
        // "A 32-bit normal compression format."    https://docs.microsoft.com/en-us/previous-versions/windows/desktop/bb322854(v=vs.85)
        // (CxV8U8              16, (?,?)),

        (A1,                     1, (1,1)),
        (A2B10G10R10_XR_BIAS,   32, (1,1)),
        //(BinaryBuffer,        ??, (?,?)), // not pixel data
    }
}
