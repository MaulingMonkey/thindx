#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_srv_dimension)\]
/// D3D_SRV_DIMENSION
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SrvDimension(D3D_SRV_DIMENSION);

// Note: D3D10_*   aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D10_1_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_*   aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! {
    SrvDimension => D3D_SRV_DIMENSION;
    Unknown, Buffer, Texture1D, Texture1DArray, Texture2D, Texture2DArray, Texture2DMS, Texture2DMSArray, Texture3D,
    TextureCube, TextureCubeArray, BufferEx,
}

#[allow(non_upper_case_globals)] impl SrvDimension { // These are enum-like
    pub const Unknown           : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_UNKNOWN);
    pub const Buffer            : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_BUFFER);
    pub const Texture1D         : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE1D);
    pub const Texture1DArray    : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE1DARRAY);
    pub const Texture2D         : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE2D);
    pub const Texture2DArray    : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE2DARRAY);
    pub const Texture2DMS       : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE2DMS);
    pub const Texture2DMSArray  : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE2DMSARRAY);
    pub const Texture3D         : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE3D);
    pub const TextureCube       : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURECUBE);
    pub const TextureCubeArray  : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURECUBEARRAY);
    pub const BufferEx          : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_BUFFEREX);
}

#[doc(hidden)] impl SrvDimension { // Ctrl+C Ctrl+V support
    pub const UNKNOWN           : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_UNKNOWN);
    pub const BUFFER            : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_BUFFER);
    pub const TEXTURE1D         : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE1D);
    pub const TEXTURE1DARRAY    : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE1DARRAY);
    pub const TEXTURE2D         : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE2D);
    pub const TEXTURE2DARRAY    : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE2DARRAY);
    pub const TEXTURE2DMS       : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE2DMS);
    pub const TEXTURE2DMSARRAY  : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE2DMSARRAY);
    pub const TEXTURE3D         : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURE3D);
    pub const TEXTURECUBE       : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURECUBE);
    pub const TEXTURECUBEARRAY  : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_TEXTURECUBEARRAY);
    pub const BUFFEREX          : SrvDimension = SrvDimension(D3D_SRV_DIMENSION_BUFFEREX);
}

impl Default for SrvDimension {
    fn default() -> Self { SrvDimension(0) }
}
