#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_type)\]
/// D3D_SHADER_VARIABLE_TYPE / D3D_SVT_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderVariableType(D3D_SHADER_VARIABLE_TYPE);
#[doc(hidden)] pub use ShaderVariableType as SVT;

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! {
    SVT => D3D_SHADER_VARIABLE_TYPE;
    Void, Bool, Int, Float, String, Texture, Texture1D, Texture2D, Texture3D, TextureCube, Sampler, Sampler1D, Sampler2D,
    Sampler3D, SamplerCube, PixelShader, VertexShader, PixelFragment, VertexFragment, UInt, UInt8, GeometryShader,
    Rasterizer, DepthStencil, Blend, Buffer, CBuffer, TBuffer, Texture1DArray, Texture2DArray, RenderTargetView,
    DepthStencilView, Texture2DMS, Texture2DMSArray, TextureCubeArray, HullShader, DomainShader, InterfacePointer,
    ComputeShader, Double, RWTexture1D, RWTexture1DArray, RWTexture2D, RWTexture2DArray, RWTexture3D, RWBuffer,
    ByteAddressBuffer, RWByteAddressBuffer, StructuredBuffer, RWStructuredBuffer, AppendStructuredBuffer,
    ConsumeStructuredBuffer, Min8Float, Min10Float, Min16Float, Min12Int, Min16Int, Min16UInt,
}

#[allow(non_upper_case_globals)] impl SVT { // These are enum-like
    pub const Void                      : SVT = SVT(D3D_SVT_VOID);
    pub const Bool                      : SVT = SVT(D3D_SVT_BOOL);
    pub const Int                       : SVT = SVT(D3D_SVT_INT);
    pub const Float                     : SVT = SVT(D3D_SVT_FLOAT);
    pub const String                    : SVT = SVT(D3D_SVT_STRING);
    pub const Texture                   : SVT = SVT(D3D_SVT_TEXTURE);
    pub const Texture1D                 : SVT = SVT(D3D_SVT_TEXTURE1D);
    pub const Texture2D                 : SVT = SVT(D3D_SVT_TEXTURE2D);
    pub const Texture3D                 : SVT = SVT(D3D_SVT_TEXTURE3D);
    pub const TextureCube               : SVT = SVT(D3D_SVT_TEXTURECUBE);
    pub const Sampler                   : SVT = SVT(D3D_SVT_SAMPLER);
    pub const Sampler1D                 : SVT = SVT(D3D_SVT_SAMPLER1D);
    pub const Sampler2D                 : SVT = SVT(D3D_SVT_SAMPLER2D);
    pub const Sampler3D                 : SVT = SVT(D3D_SVT_SAMPLER3D);
    pub const SamplerCube               : SVT = SVT(D3D_SVT_SAMPLERCUBE);
    pub const PixelShader               : SVT = SVT(D3D_SVT_PIXELSHADER);
    pub const VertexShader              : SVT = SVT(D3D_SVT_VERTEXSHADER);
    pub const PixelFragment             : SVT = SVT(D3D_SVT_PIXELFRAGMENT);
    pub const VertexFragment            : SVT = SVT(D3D_SVT_VERTEXFRAGMENT);
    pub const UInt                      : SVT = SVT(D3D_SVT_UINT);
    pub const UInt8                     : SVT = SVT(D3D_SVT_UINT8);
    pub const GeometryShader            : SVT = SVT(D3D_SVT_GEOMETRYSHADER);
    pub const Rasterizer                : SVT = SVT(D3D_SVT_RASTERIZER);
    pub const DepthStencil              : SVT = SVT(D3D_SVT_DEPTHSTENCIL);
    pub const Blend                     : SVT = SVT(D3D_SVT_BLEND);
    pub const Buffer                    : SVT = SVT(D3D_SVT_BUFFER);
    pub const CBuffer                   : SVT = SVT(D3D_SVT_CBUFFER);
    pub const TBuffer                   : SVT = SVT(D3D_SVT_TBUFFER);
    pub const Texture1DArray            : SVT = SVT(D3D_SVT_TEXTURE1DARRAY);
    pub const Texture2DArray            : SVT = SVT(D3D_SVT_TEXTURE2DARRAY);
    pub const RenderTargetView          : SVT = SVT(D3D_SVT_RENDERTARGETVIEW);
    pub const DepthStencilView          : SVT = SVT(D3D_SVT_DEPTHSTENCILVIEW);
    pub const Texture2DMS               : SVT = SVT(D3D_SVT_TEXTURE2DMS);
    pub const Texture2DMSArray          : SVT = SVT(D3D_SVT_TEXTURE2DMSARRAY);
    pub const TextureCubeArray          : SVT = SVT(D3D_SVT_TEXTURECUBEARRAY);
    pub const HullShader                : SVT = SVT(D3D_SVT_HULLSHADER);
    pub const DomainShader              : SVT = SVT(D3D_SVT_DOMAINSHADER);
    pub const InterfacePointer          : SVT = SVT(D3D_SVT_INTERFACE_POINTER);
    pub const ComputeShader             : SVT = SVT(D3D_SVT_COMPUTESHADER);
    pub const Double                    : SVT = SVT(D3D_SVT_DOUBLE);
    pub const RWTexture1D               : SVT = SVT(D3D_SVT_RWTEXTURE1D);
    pub const RWTexture1DArray          : SVT = SVT(D3D_SVT_RWTEXTURE1DARRAY);
    pub const RWTexture2D               : SVT = SVT(D3D_SVT_RWTEXTURE2D);
    pub const RWTexture2DArray          : SVT = SVT(D3D_SVT_RWTEXTURE2DARRAY);
    pub const RWTexture3D               : SVT = SVT(D3D_SVT_RWTEXTURE3D);
    pub const RWBuffer                  : SVT = SVT(D3D_SVT_RWBUFFER);
    pub const ByteAddressBuffer         : SVT = SVT(D3D_SVT_BYTEADDRESS_BUFFER);
    pub const RWByteAddressBuffer       : SVT = SVT(D3D_SVT_RWBYTEADDRESS_BUFFER);
    pub const StructuredBuffer          : SVT = SVT(D3D_SVT_STRUCTURED_BUFFER);
    pub const RWStructuredBuffer        : SVT = SVT(D3D_SVT_RWSTRUCTURED_BUFFER);
    pub const AppendStructuredBuffer    : SVT = SVT(D3D_SVT_APPEND_STRUCTURED_BUFFER);
    pub const ConsumeStructuredBuffer   : SVT = SVT(D3D_SVT_CONSUME_STRUCTURED_BUFFER);
    pub const Min8Float                 : SVT = SVT(D3D_SVT_MIN8FLOAT);
    pub const Min10Float                : SVT = SVT(D3D_SVT_MIN10FLOAT);
    pub const Min16Float                : SVT = SVT(D3D_SVT_MIN16FLOAT);
    pub const Min12Int                  : SVT = SVT(D3D_SVT_MIN12INT);
    pub const Min16Int                  : SVT = SVT(D3D_SVT_MIN16INT);
    pub const Min16UInt                 : SVT = SVT(D3D_SVT_MIN16UINT);
}

#[doc(hidden)] impl SVT { // Ctrl+C Ctrl+V support
    pub const VOID                      : SVT = SVT(D3D_SVT_VOID);
    pub const BOOL                      : SVT = SVT(D3D_SVT_BOOL);
    pub const INT                       : SVT = SVT(D3D_SVT_INT);
    pub const FLOAT                     : SVT = SVT(D3D_SVT_FLOAT);
    pub const STRING                    : SVT = SVT(D3D_SVT_STRING);
    pub const TEXTURE                   : SVT = SVT(D3D_SVT_TEXTURE);
    pub const TEXTURE1D                 : SVT = SVT(D3D_SVT_TEXTURE1D);
    pub const TEXTURE2D                 : SVT = SVT(D3D_SVT_TEXTURE2D);
    pub const TEXTURE3D                 : SVT = SVT(D3D_SVT_TEXTURE3D);
    pub const TEXTURECUBE               : SVT = SVT(D3D_SVT_TEXTURECUBE);
    pub const SAMPLER                   : SVT = SVT(D3D_SVT_SAMPLER);
    pub const SAMPLER1D                 : SVT = SVT(D3D_SVT_SAMPLER1D);
    pub const SAMPLER2D                 : SVT = SVT(D3D_SVT_SAMPLER2D);
    pub const SAMPLER3D                 : SVT = SVT(D3D_SVT_SAMPLER3D);
    pub const SAMPLERCUBE               : SVT = SVT(D3D_SVT_SAMPLERCUBE);
    pub const PIXELSHADER               : SVT = SVT(D3D_SVT_PIXELSHADER);
    pub const VERTEXSHADER              : SVT = SVT(D3D_SVT_VERTEXSHADER);
    pub const PIXELFRAGMENT             : SVT = SVT(D3D_SVT_PIXELFRAGMENT);
    pub const VERTEXFRAGMENT            : SVT = SVT(D3D_SVT_VERTEXFRAGMENT);
    pub const UINT                      : SVT = SVT(D3D_SVT_UINT);
    pub const UINT8                     : SVT = SVT(D3D_SVT_UINT8);
    pub const GEOMETRYSHADER            : SVT = SVT(D3D_SVT_GEOMETRYSHADER);
    pub const RASTERIZER                : SVT = SVT(D3D_SVT_RASTERIZER);
    pub const DEPTHSTENCIL              : SVT = SVT(D3D_SVT_DEPTHSTENCIL);
    pub const BLEND                     : SVT = SVT(D3D_SVT_BLEND);
    pub const BUFFER                    : SVT = SVT(D3D_SVT_BUFFER);
    pub const CBUFFER                   : SVT = SVT(D3D_SVT_CBUFFER);
    pub const TBUFFER                   : SVT = SVT(D3D_SVT_TBUFFER);
    pub const TEXTURE1DARRAY            : SVT = SVT(D3D_SVT_TEXTURE1DARRAY);
    pub const TEXTURE2DARRAY            : SVT = SVT(D3D_SVT_TEXTURE2DARRAY);
    pub const RENDERTARGETVIEW          : SVT = SVT(D3D_SVT_RENDERTARGETVIEW);
    pub const DEPTHSTENCILVIEW          : SVT = SVT(D3D_SVT_DEPTHSTENCILVIEW);
    pub const TEXTURE2DMS               : SVT = SVT(D3D_SVT_TEXTURE2DMS);
    pub const TEXTURE2DMSARRAY          : SVT = SVT(D3D_SVT_TEXTURE2DMSARRAY);
    pub const TEXTURECUBEARRAY          : SVT = SVT(D3D_SVT_TEXTURECUBEARRAY);
    pub const HULLSHADER                : SVT = SVT(D3D_SVT_HULLSHADER);
    pub const DOMAINSHADER              : SVT = SVT(D3D_SVT_DOMAINSHADER);
    pub const INTERFACE_POINTER         : SVT = SVT(D3D_SVT_INTERFACE_POINTER);
    pub const COMPUTESHADER             : SVT = SVT(D3D_SVT_COMPUTESHADER);
    pub const DOUBLE                    : SVT = SVT(D3D_SVT_DOUBLE);
    pub const RWTEXTURE1D               : SVT = SVT(D3D_SVT_RWTEXTURE1D);
    pub const RWTEXTURE1DARRAY          : SVT = SVT(D3D_SVT_RWTEXTURE1DARRAY);
    pub const RWTEXTURE2D               : SVT = SVT(D3D_SVT_RWTEXTURE2D);
    pub const RWTEXTURE2DARRAY          : SVT = SVT(D3D_SVT_RWTEXTURE2DARRAY);
    pub const RWTEXTURE3D               : SVT = SVT(D3D_SVT_RWTEXTURE3D);
    pub const RWBUFFER                  : SVT = SVT(D3D_SVT_RWBUFFER);
    pub const BYTEADDRESS_BUFFER        : SVT = SVT(D3D_SVT_BYTEADDRESS_BUFFER);
    pub const RWBYTEADDRESS_BUFFER      : SVT = SVT(D3D_SVT_RWBYTEADDRESS_BUFFER);
    pub const STRUCTURED_BUFFER         : SVT = SVT(D3D_SVT_STRUCTURED_BUFFER);
    pub const RWSTRUCTURED_BUFFER       : SVT = SVT(D3D_SVT_RWSTRUCTURED_BUFFER);
    pub const APPEND_STRUCTURED_BUFFER  : SVT = SVT(D3D_SVT_APPEND_STRUCTURED_BUFFER);
    pub const CONSUME_STRUCTURED_BUFFER : SVT = SVT(D3D_SVT_CONSUME_STRUCTURED_BUFFER);
    pub const MIN8FLOAT                 : SVT = SVT(D3D_SVT_MIN8FLOAT);
    pub const MIN10FLOAT                : SVT = SVT(D3D_SVT_MIN10FLOAT);
    pub const MIN16FLOAT                : SVT = SVT(D3D_SVT_MIN16FLOAT);
    pub const MIN12INT                  : SVT = SVT(D3D_SVT_MIN12INT);
    pub const MIN16INT                  : SVT = SVT(D3D_SVT_MIN16INT);
    pub const MIN16UINT                 : SVT = SVT(D3D_SVT_MIN16UINT);
}

impl Default for SVT {
    fn default() -> Self { SVT(0) }
}
