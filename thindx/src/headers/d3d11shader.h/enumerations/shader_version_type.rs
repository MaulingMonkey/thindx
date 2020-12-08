#[allow(unused_imports)] use crate::*;

use winapi::um::d3d11shader::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ne-d3d11shader-d3d11_shader_version_type)\]
/// D3D11_SHADER_VERSION_TYPE / D3D11_SHVER_\*
///
/// ### See Also
/// *   [ShaderVersion]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderVersionType(D3D11_SHADER_VERSION_TYPE);
#[doc(hidden)] pub use ShaderVersionType as ShVer;

enumish! { ShVer => D3D11_SHADER_VERSION_TYPE; PixelShader, VertexShader, GeometryShader, HullShader, DomainShader, ComputeShader, Reserved0 }

#[allow(non_upper_case_globals)] impl ShVer { // These are enum-like
    pub const PixelShader       : ShVer = ShVer(D3D11_SHVER_PIXEL_SHADER); // 0
    pub const VertexShader      : ShVer = ShVer(D3D11_SHVER_VERTEX_SHADER);
    pub const GeometryShader    : ShVer = ShVer(D3D11_SHVER_GEOMETRY_SHADER);
    pub const HullShader        : ShVer = ShVer(D3D11_SHVER_HULL_SHADER);
    pub const DomainShader      : ShVer = ShVer(D3D11_SHVER_DOMAIN_SHADER);
    pub const ComputeShader     : ShVer = ShVer(D3D11_SHVER_COMPUTE_SHADER);
    pub const Reserved0         : ShVer = ShVer(D3D11_SHVER_RESERVED0);
}

#[doc(hidden)] impl ShVer { // Ctrl+C Ctrl+V support
    pub const PIXEL_SHADER      : ShVer = ShVer(D3D11_SHVER_PIXEL_SHADER); // 0
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



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ne-d3d11shader-d3d11_shader_version_type)\]
/// UINT mask containing [ShaderVersionType]
///
/// ### See Also
/// *   [d3d11::ShaderDesc::version]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderVersion(u32);

impl ShaderVersion {
    /// D3D11_SHVER_GET_TYPE
    pub fn shver(&self) -> ShaderVersionType { ShaderVersionType((self.0 >> 16) & 0xFFFF) }

    /// D3D11_SHVER_GET_MAJOR
    pub fn major(&self) -> u32 { (self.0 >> 4) & 0xF }

    /// D3D11_SHVER_GET_MINOR
    pub fn minor(&self) -> u32 { (self.0 >> 0) & 0xF }
}

impl Debug for ShaderVersion {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let (ty, maj, min) = (self.shver(), self.major(), self.minor());
        match ty {
            ShVer::PixelShader      => write!(fmt, "ps_{}_{}", maj, min),
            ShVer::VertexShader     => write!(fmt, "vs_{}_{}", maj, min),
            ShVer::GeometryShader   => write!(fmt, "gs_{}_{}", maj, min),
            ShVer::HullShader       => write!(fmt, "hs_{}_{}", maj, min),
            ShVer::DomainShader     => write!(fmt, "ds_{}_{}", maj, min),
            ShVer::ComputeShader    => write!(fmt, "cs_{}_{}", maj, min),
            _other                  => fmt.debug_struct("ShaderVersion").field("shver", &ty).field("major", &maj).field("minor", &min).finish()
        }
    }
}
