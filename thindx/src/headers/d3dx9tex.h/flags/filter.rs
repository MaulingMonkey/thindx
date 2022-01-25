// missing from winapi:
#[allow(non_camel_case_types)] type D3DX_FILTER = u32;

const D3DX_FILTER_NONE              : u32 = 1 << 0;
const D3DX_FILTER_POINT             : u32 = 2 << 0;
const D3DX_FILTER_LINEAR            : u32 = 3 << 0;
const D3DX_FILTER_TRIANGLE          : u32 = 4 << 0;
const D3DX_FILTER_BOX               : u32 = 5 << 0;

const D3DX_FILTER_MIRROR_U          : u32 = 1 << 16;
const D3DX_FILTER_MIRROR_V          : u32 = 2 << 16;
const D3DX_FILTER_MIRROR_W          : u32 = 4 << 16;
const D3DX_FILTER_MIRROR            : u32 = 7 << 16;

const D3DX_FILTER_DITHER            : u32 = 1 << 19;
const D3DX_FILTER_DITHER_DIFFUSION  : u32 = 2 << 19;

const D3DX_FILTER_SRGB_IN           : u32 = 1 << 21;
const D3DX_FILTER_SRGB_OUT          : u32 = 2 << 21;
const D3DX_FILTER_SRGB              : u32 = 3 << 21;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dx-filter)\]
/// D3DX_FILTER
///
/// Used to specify which channels in a texture to operate on.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Filter(D3DX_FILTER);

flags! { Filter => D3DX_FILTER;
    // This is more like a packed bitfield, so I reverse order from highest bit set to least
    Srgb, SrgbOut, SrgbIn,
    DitherDiffusion, Dither,
    Mirror, MirrorW, MirrorV, MirrorU,
    Box, Triangle, Linear, Point, None,
}

#[allow(non_upper_case_globals)] impl Filter {

    /// No scaling or filtering will take place.  Pixels outside the bounds
    /// of the source image are assumed to be transparent black.
    pub const None             : Filter = Filter(D3DX_FILTER_NONE);

    /// Each destination pixel is computed by sampling the nearest pixel
    /// from the source image.
    pub const Point            : Filter = Filter(D3DX_FILTER_POINT);

    /// Each destination pixel is computed by linearly interpolating between
    /// the nearest pixels in the source image.  This filter works best
    /// when the scale on each axis is less than 2.
    pub const Linear           : Filter = Filter(D3DX_FILTER_LINEAR);

    /// Every pixel in the source image contributes equally to the
    /// destination image.  This is the slowest of all the filters.
    pub const Triangle         : Filter = Filter(D3DX_FILTER_TRIANGLE);

    /// Each pixel is computed by averaging a 2x2(x2) box pixels from
    /// the source image. Only works when the dimensions of the
    /// destination are half those of the source. (as with mip maps)
    pub const Box              : Filter = Filter(D3DX_FILTER_BOX);



    /// Indicates that pixels off the edge of the texture on the X-axis should be mirrored, not wraped.
    pub const MirrorU          : Filter = Filter(D3DX_FILTER_MIRROR_U);

    /// Indicates that pixels off the edge of the texture on the Y-axis should be mirrored, not wraped.
    pub const MirrorV          : Filter = Filter(D3DX_FILTER_MIRROR_V);

    /// Indicates that pixels off the edge of the texture on the W-axis should be mirrored, not wraped.
    pub const MirrorW          : Filter = Filter(D3DX_FILTER_MIRROR_W);

    /// Same as specifying [MirrorU] | [MirrorV] | [MirrorW]
    pub const Mirror           : Filter = Filter(D3DX_FILTER_MIRROR);



    /// Dithers the resulting image using a 4x4 order dither pattern.
    pub const Dither           : Filter = Filter(D3DX_FILTER_DITHER);

    #[allow(missing_docs)]
    pub const DitherDiffusion  : Filter = Filter(D3DX_FILTER_DITHER_DIFFUSION);



    /// The input data is in sRGB (gamma 2.2) colorspace.
    pub const SrgbIn           : Filter = Filter(D3DX_FILTER_SRGB_IN);

    /// The output data is in sRGB (gamma 2.2) colorspace.
    pub const SrgbOut          : Filter = Filter(D3DX_FILTER_SRGB_OUT);

    /// Same as specifying [SrgbIn] | [SrgbOut]
    pub const Srgb             : Filter = Filter(D3DX_FILTER_SRGB);
}



//#cpp2rust D3DX_FILTER                     = d3dx9::Filter

//#cpp2rust D3DX_FILTER_NONE                = d3dx9::Filter::None
//#cpp2rust D3DX_FILTER_POINT               = d3dx9::Filter::Point
//#cpp2rust D3DX_FILTER_LINEAR              = d3dx9::Filter::Linear
//#cpp2rust D3DX_FILTER_TRIANGLE            = d3dx9::Filter::Triangle
//#cpp2rust D3DX_FILTER_BOX                 = d3dx9::Filter::Box

//#cpp2rust D3DX_FILTER_MIRROR_U            = d3dx9::Filter::MirrorU
//#cpp2rust D3DX_FILTER_MIRROR_V            = d3dx9::Filter::MirrorV
//#cpp2rust D3DX_FILTER_MIRROR_W            = d3dx9::Filter::MirrorW
//#cpp2rust D3DX_FILTER_MIRROR              = d3dx9::Filter::Mirror

//#cpp2rust D3DX_FILTER_DITHER              = d3dx9::Filter::Dither
//#cpp2rust D3DX_FILTER_DITHER_DIFFUSION    = d3dx9::Filter::DitherDiffusion

//#cpp2rust D3DX_FILTER_SRGB_IN             = d3dx9::Filter::SrgbIn
//#cpp2rust D3DX_FILTER_SRGB_OUT            = d3dx9::Filter::SrgbOut
//#cpp2rust D3DX_FILTER_SRGB                = d3dx9::Filter::Srgb
