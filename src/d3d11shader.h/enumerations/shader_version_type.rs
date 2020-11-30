#[allow(unused_imports)] use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d11_shader_version_type)\]
/// D3D11_SHADER_VERSION_TYPE / D3D11_SHVER_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderVersionType(D3D11_SHADER_VERSION_TYPE);
#[doc(hidden)] pub use ShaderVersionType as ShVer;

enumish! { ShVer => D3D11_SHADER_VERSION_TYPE; PixelShader, VertexShader, GeometryShader, HullShader, DomainShader, ComputeShader, Reserved0 }

#[allow(non_upper_case_globals)] impl ShVer { // These are enum-like
    pub const PixelShader       : ShVer = ShVer(D3D11_SHVER_PIXEL_SHADER);
    pub const VertexShader      : ShVer = ShVer(D3D11_SHVER_VERTEX_SHADER);
    pub const GeometryShader    : ShVer = ShVer(D3D11_SHVER_GEOMETRY_SHADER);
    pub const HullShader        : ShVer = ShVer(D3D11_SHVER_HULL_SHADER);
    pub const DomainShader      : ShVer = ShVer(D3D11_SHVER_DOMAIN_SHADER);
    pub const ComputeShader     : ShVer = ShVer(D3D11_SHVER_COMPUTE_SHADER);
    pub const Reserved0         : ShVer = ShVer(D3D11_SHVER_RESERVED0);
}

#[doc(hidden)] impl ShVer { // Ctrl+C Ctrl+V support
    pub const PIXEL_SHADER      : ShVer = ShVer(D3D11_SHVER_PIXEL_SHADER);
    pub const VERTEX_SHADER     : ShVer = ShVer(D3D11_SHVER_VERTEX_SHADER);
    pub const GEOMETRY_SHADER   : ShVer = ShVer(D3D11_SHVER_GEOMETRY_SHADER);
    pub const HULL_SHADER       : ShVer = ShVer(D3D11_SHVER_HULL_SHADER);
    pub const DOMAIN_SHADER     : ShVer = ShVer(D3D11_SHVER_DOMAIN_SHADER);
    pub const COMPUTE_SHADER    : ShVer = ShVer(D3D11_SHVER_COMPUTE_SHADER);
    pub const RESERVED0         : ShVer = ShVer(D3D11_SHVER_RESERVED0);
}

impl Default for ShVer {
    fn default() -> Self { ShVer(0) }
}
