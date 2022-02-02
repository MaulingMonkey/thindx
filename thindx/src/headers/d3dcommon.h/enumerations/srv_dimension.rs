#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_srv_dimension)\]
/// D3D_SRV_DIMENSION
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct SrvDimension(D3D_SRV_DIMENSION);

// Note: D3D10_*   aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D10_1_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_*   aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! {
    SrvDimension => D3D_SRV_DIMENSION;
    Unknown, Buffer, Texture1D, Texture1DArray, Texture2D, Texture2DArray, Texture2DMS, Texture2DMSArray, Texture3D,
    TextureCube, TextureCubeArray, BufferEx,
}

#[allow(missing_docs)]
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

//#cpp2rust D3D_SRV_DIMENSION                       = d3d::SrvDimension

//#cpp2rust D3D_SRV_DIMENSION_UNKNOWN               = d3d::SrvDimension::Unknown
//#cpp2rust D3D_SRV_DIMENSION_BUFFER                = d3d::SrvDimension::Buffer
//#cpp2rust D3D_SRV_DIMENSION_TEXTURE1D             = d3d::SrvDimension::Texture1D
//#cpp2rust D3D_SRV_DIMENSION_TEXTURE1DARRAY        = d3d::SrvDimension::Texture1DArray
//#cpp2rust D3D_SRV_DIMENSION_TEXTURE2D             = d3d::SrvDimension::Texture2D
//#cpp2rust D3D_SRV_DIMENSION_TEXTURE2DARRAY        = d3d::SrvDimension::Texture2DArray
//#cpp2rust D3D_SRV_DIMENSION_TEXTURE2DMS           = d3d::SrvDimension::Texture2DMS
//#cpp2rust D3D_SRV_DIMENSION_TEXTURE2DMSARRAY      = d3d::SrvDimension::Texture2DMSArray
//#cpp2rust D3D_SRV_DIMENSION_TEXTURE3D             = d3d::SrvDimension::Texture3D
//#cpp2rust D3D_SRV_DIMENSION_TEXTURECUBE           = d3d::SrvDimension::TextureCube
//#cpp2rust D3D_SRV_DIMENSION_TEXTURECUBEARRAY      = d3d::SrvDimension::TextureCubeArray
//#cpp2rust D3D_SRV_DIMENSION_BUFFEREX              = d3d::SrvDimension::BufferEx

//#cpp2rust D3D10_1_SRV_DIMENSION_UNKNOWN           = d3d::SrvDimension::Unknown
//#cpp2rust D3D10_1_SRV_DIMENSION_BUFFER            = d3d::SrvDimension::Buffer
//#cpp2rust D3D10_1_SRV_DIMENSION_TEXTURE1D         = d3d::SrvDimension::Texture1D
//#cpp2rust D3D10_1_SRV_DIMENSION_TEXTURE1DARRAY    = d3d::SrvDimension::Texture1DArray
//#cpp2rust D3D10_1_SRV_DIMENSION_TEXTURE2D         = d3d::SrvDimension::Texture2D
//#cpp2rust D3D10_1_SRV_DIMENSION_TEXTURE2DARRAY    = d3d::SrvDimension::Texture2DArray
//#cpp2rust D3D10_1_SRV_DIMENSION_TEXTURE2DMS       = d3d::SrvDimension::Texture2DMS
//#cpp2rust D3D10_1_SRV_DIMENSION_TEXTURE2DMSARRAY  = d3d::SrvDimension::Texture2DMSArray
//#cpp2rust D3D10_1_SRV_DIMENSION_TEXTURE3D         = d3d::SrvDimension::Texture3D
//#cpp2rust D3D10_1_SRV_DIMENSION_TEXTURECUBE       = d3d::SrvDimension::TextureCube
//#cpp2rust D3D10_1_SRV_DIMENSION_TEXTURECUBEARRAY  = d3d::SrvDimension::TextureCubeArray

//#cpp2rust D3D10_SRV_DIMENSION_UNKNOWN             = d3d::SrvDimension::Unknown
//#cpp2rust D3D10_SRV_DIMENSION_BUFFER              = d3d::SrvDimension::Buffer
//#cpp2rust D3D10_SRV_DIMENSION_TEXTURE1D           = d3d::SrvDimension::Texture1D
//#cpp2rust D3D10_SRV_DIMENSION_TEXTURE1DARRAY      = d3d::SrvDimension::Texture1DArray
//#cpp2rust D3D10_SRV_DIMENSION_TEXTURE2D           = d3d::SrvDimension::Texture2D
//#cpp2rust D3D10_SRV_DIMENSION_TEXTURE2DARRAY      = d3d::SrvDimension::Texture2DArray
//#cpp2rust D3D10_SRV_DIMENSION_TEXTURE2DMS         = d3d::SrvDimension::Texture2DMS
//#cpp2rust D3D10_SRV_DIMENSION_TEXTURE2DMSARRAY    = d3d::SrvDimension::Texture2DMSArray
//#cpp2rust D3D10_SRV_DIMENSION_TEXTURE3D           = d3d::SrvDimension::Texture3D
//#cpp2rust D3D10_SRV_DIMENSION_TEXTURECUBE         = d3d::SrvDimension::TextureCube
//#cpp2rust D3D10_SRV_DIMENSION_TEXTURECUBEARRAY    = d3d::SrvDimension::TextureCubeArray

//#cpp2rust D3D11_SRV_DIMENSION_UNKNOWN             = d3d::SrvDimension::Unknown
//#cpp2rust D3D11_SRV_DIMENSION_BUFFER              = d3d::SrvDimension::Buffer
//#cpp2rust D3D11_SRV_DIMENSION_TEXTURE1D           = d3d::SrvDimension::Texture1D
//#cpp2rust D3D11_SRV_DIMENSION_TEXTURE1DARRAY      = d3d::SrvDimension::Texture1DArray
//#cpp2rust D3D11_SRV_DIMENSION_TEXTURE2D           = d3d::SrvDimension::Texture2D
//#cpp2rust D3D11_SRV_DIMENSION_TEXTURE2DARRAY      = d3d::SrvDimension::Texture2DArray
//#cpp2rust D3D11_SRV_DIMENSION_TEXTURE2DMS         = d3d::SrvDimension::Texture2DMS
//#cpp2rust D3D11_SRV_DIMENSION_TEXTURE2DMSARRAY    = d3d::SrvDimension::Texture2DMSArray
//#cpp2rust D3D11_SRV_DIMENSION_TEXTURE3D           = d3d::SrvDimension::Texture3D
//#cpp2rust D3D11_SRV_DIMENSION_TEXTURECUBE         = d3d::SrvDimension::TextureCube
//#cpp2rust D3D11_SRV_DIMENSION_TEXTURECUBEARRAY    = d3d::SrvDimension::TextureCubeArray
//#cpp2rust D3D11_SRV_DIMENSION_BUFFEREX            = d3d::SrvDimension::BufferEx
