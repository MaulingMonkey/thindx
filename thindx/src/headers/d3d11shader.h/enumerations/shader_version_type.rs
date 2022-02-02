#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3d11shader::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ne-d3d11shader-d3d11_shader_version_type)\]
/// D3D11_SHADER_VERSION_TYPE / D3D11_SHVER_\*
///
/// ### Example
/// ```rust
/// # use thindx::{d3d::*, d3d11::*};
/// # let d3dc = Compiler::load_system(47).unwrap();
/// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
/// let shader = d3dc.reflect11(&shader).unwrap();
/// let ty : ShaderVersionType = shader.get_desc().unwrap().version.ty();
/// assert_eq!(ShVer::PixelShader, ty);
/// ```
///
/// ### See Also
/// *   [d3d11::ShaderDesc::version]
/// *   [d3d11::ShaderReflection::get_desc]
/// *   [d3d11::ShaderVersion::ty]
/// *   [examples::d3dcompiler_04_reflect_shader]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ShaderVersionType(D3D11_SHADER_VERSION_TYPE);
#[doc(hidden)] pub use ShaderVersionType as ShVer;

enumish! { ShVer => D3D11_SHADER_VERSION_TYPE; PixelShader, VertexShader, GeometryShader, HullShader, DomainShader, ComputeShader, Reserved0 }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl ShVer { // These are enum-like
    pub const PixelShader       : ShVer = ShVer(D3D11_SHVER_PIXEL_SHADER); // 0
    pub const VertexShader      : ShVer = ShVer(D3D11_SHVER_VERTEX_SHADER);
    pub const GeometryShader    : ShVer = ShVer(D3D11_SHVER_GEOMETRY_SHADER);
    pub const HullShader        : ShVer = ShVer(D3D11_SHVER_HULL_SHADER);
    pub const DomainShader      : ShVer = ShVer(D3D11_SHVER_DOMAIN_SHADER);
    pub const ComputeShader     : ShVer = ShVer(D3D11_SHVER_COMPUTE_SHADER);
    #[doc=""]
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



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ne-d3d11shader-d3d11_shader_version_type)\]
/// UINT mask of ([ShaderVersionType] << 16) | (major << 4) | (minor << 0)
///
/// ### Example
/// ```rust
/// # use thindx::{d3d::*, d3d11::*, *};
/// # let d3dc = Compiler::load_system(47).unwrap();
/// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
/// let shader = d3dc.reflect11(&shader).unwrap();
/// let version : d3d11::ShaderVersion = shader.get_desc().unwrap().version;
/// assert_eq!(version.ty(), ShVer::PixelShader);
/// assert_eq!(version.major(), 4);
/// assert_eq!(version.minor(), 0);
/// assert_eq!(format!("{:?}", version), "ps_4_0");
/// ```
///
/// ### See Also
/// *   [d3d11::ShaderDesc::version]
/// *   [d3d11::ShaderReflection::get_desc]
/// *   [examples::d3dcompiler_04_reflect_shader]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ShaderVersion(u32);

impl ShaderVersion {
    /// D3D11_SHVER_GET_TYPE
    pub fn ty(&self) -> ShaderVersionType { ShaderVersionType((self.0 >> 16) & 0xFFFF) }

    /// D3D11_SHVER_GET_MAJOR
    pub fn major(&self) -> u32 { (self.0 >> 4) & 0xF }

    /// D3D11_SHVER_GET_MINOR
    pub fn minor(&self) -> u32 { (self.0 >> 0) & 0xF }
}

impl Debug for ShaderVersion {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let (ty, maj, min) = (self.ty(), self.major(), self.minor());
        match ty {
            ShVer::PixelShader      => write!(fmt, "ps_{}_{}", maj, min),
            ShVer::VertexShader     => write!(fmt, "vs_{}_{}", maj, min),
            ShVer::GeometryShader   => write!(fmt, "gs_{}_{}", maj, min),
            ShVer::HullShader       => write!(fmt, "hs_{}_{}", maj, min),
            ShVer::DomainShader     => write!(fmt, "ds_{}_{}", maj, min),
            ShVer::ComputeShader    => write!(fmt, "cs_{}_{}", maj, min),
            _other                  => fmt.debug_struct("ShaderVersion").field("ty", &ty).field("major", &maj).field("minor", &min).finish()
        }
    }
}

//#cpp2rust D3D11_SHVER_GET_TYPE            = d3d11::ShaderVersion::ty
//#cpp2rust D3D11_SHVER_GET_MAJOR           = d3d11::ShaderVersion::major
//#cpp2rust D3D11_SHVER_GET_MINOR           = d3d11::ShaderVersion::minor

//#cpp2rust D3D11_SHADER_VERSION_TYPE       = d3d11::ShaderVersionType

//#cpp2rust D3D11_SHVER_PIXEL_SHADER        = d3d11::ShVer::PixelShader
//#cpp2rust D3D11_SHVER_VERTEX_SHADER       = d3d11::ShVer::VertexShader
//#cpp2rust D3D11_SHVER_GEOMETRY_SHADER     = d3d11::ShVer::GeometryShader
//#cpp2rust D3D11_SHVER_HULL_SHADER         = d3d11::ShVer::HullShader
//#cpp2rust D3D11_SHVER_DOMAIN_SHADER       = d3d11::ShVer::DomainShader
//#cpp2rust D3D11_SHVER_COMPUTE_SHADER      = d3d11::ShVer::ComputeShader
//#cpp2rust D3D11_SHVER_RESERVED0           = d3d11::ShVer::Reserved0
