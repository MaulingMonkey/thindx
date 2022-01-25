// missing from winapi:
#[allow(non_camel_case_types)] type D3DX_NORMALMAP = u32;
const D3DX_NORMALMAP_MIRROR_U           : u32 =  1 << 16;
const D3DX_NORMALMAP_MIRROR_V           : u32 =  2 << 16;
const D3DX_NORMALMAP_MIRROR             : u32 =  3 << 16;
const D3DX_NORMALMAP_INVERTSIGN         : u32 =  8 << 16;
const D3DX_NORMALMAP_COMPUTE_OCCLUSION  : u32 = 16 << 16;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dx-normalmap)\]
/// D3DX_NORMALMAP
///
/// These flags are used to control how [d3dx9::compute_normal_maps] generates normal maps.
/// Any number of these flags may be OR'd together in any combination.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct NormalMap(D3DX_NORMALMAP);

flags! { NormalMap => D3DX_NORMALMAP;
    None, Mirror,
    MirrorU, MirrorV, InvertSign, ComputeOcclusion,
}

#[allow(non_upper_case_globals)] impl NormalMap {
    /// No flags set.
    pub const None              : NormalMap = NormalMap(0);

    /// Indicates that pixels off the edge of the texture on the U-axis should be mirrored, not wraped.
    pub const MirrorU           : NormalMap = NormalMap(D3DX_NORMALMAP_MIRROR_U);

    /// Indicates that pixels off the edge of the texture on the V-axis should be mirrored, not wraped.
    pub const MirrorV           : NormalMap = NormalMap(D3DX_NORMALMAP_MIRROR_V);

    /// Same as specifying D3DX_NORMALMAP_MIRROR_U | D3DX_NORMALMAP_MIRROR_V
    pub const Mirror            : NormalMap = NormalMap(D3DX_NORMALMAP_MIRROR);

    /// Inverts the direction of each normal
    pub const InvertSign        : NormalMap = NormalMap(D3DX_NORMALMAP_INVERTSIGN);

    /// Compute the per pixel Occlusion term and encodes it into the alpha.
    /// An Alpha of 1 means that the pixel is not obscured in anyway, and
    /// an alpha of 0 would mean that the pixel is completly obscured.
    pub const ComputeOcclusion  : NormalMap = NormalMap(D3DX_NORMALMAP_COMPUTE_OCCLUSION);
}



//#cpp2rust D3DX_NORMALMAP                      = d3dx9::NormalMap

//#cpp2rust D3DX_NORMALMAP_MIRROR_U             = d3dx9::NormalMap::MirrorU
//#cpp2rust D3DX_NORMALMAP_MIRROR_V             = d3dx9::NormalMap::MirrorV
//#cpp2rust D3DX_NORMALMAP_MIRROR               = d3dx9::NormalMap::Mirror
//#cpp2rust D3DX_NORMALMAP_INVERTSIGN           = d3dx9::NormalMap::InvertSign
//#cpp2rust D3DX_NORMALMAP_COMPUTE_OCCLUSION    = d3dx9::NormalMap::ComputeOcclusion
