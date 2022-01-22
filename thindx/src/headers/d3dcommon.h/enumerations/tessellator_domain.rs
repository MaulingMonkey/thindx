#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_domain)\]
/// D3D_TESSELLATOR_DOMAIN
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TessellatorDomain(D3D_TESSELLATOR_DOMAIN);

// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { TessellatorDomain => D3D_TESSELLATOR_DOMAIN; Undefined, IsoLine, Tri, Quad }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl TessellatorDomain { // These are enum-like
    pub const Undefined : TessellatorDomain = TessellatorDomain(D3D_TESSELLATOR_DOMAIN_UNDEFINED);
    pub const IsoLine   : TessellatorDomain = TessellatorDomain(D3D_TESSELLATOR_DOMAIN_ISOLINE);
    pub const Tri       : TessellatorDomain = TessellatorDomain(D3D_TESSELLATOR_DOMAIN_TRI);
    pub const Quad      : TessellatorDomain = TessellatorDomain(D3D_TESSELLATOR_DOMAIN_QUAD);
}

#[doc(hidden)] impl TessellatorDomain { // Ctrl+C Ctrl+V support
    pub const UNDEFINED : TessellatorDomain = TessellatorDomain(D3D_TESSELLATOR_DOMAIN_UNDEFINED);
    pub const ISOLINE   : TessellatorDomain = TessellatorDomain(D3D_TESSELLATOR_DOMAIN_ISOLINE);
    pub const TRI       : TessellatorDomain = TessellatorDomain(D3D_TESSELLATOR_DOMAIN_TRI);
    pub const QUAD      : TessellatorDomain = TessellatorDomain(D3D_TESSELLATOR_DOMAIN_QUAD);
}

impl Default for TessellatorDomain {
    fn default() -> Self { TessellatorDomain(0) }
}

//#cpp2rust D3D_TESSELLATOR_DOMAIN              = d3d::TessellatorDomain

//#cpp2rust D3D_TESSELLATOR_DOMAIN_UNDEFINED    = d3d::TessellatorDomain::Undefined
//#cpp2rust D3D_TESSELLATOR_DOMAIN_ISOLINE      = d3d::TessellatorDomain::IsoLine
//#cpp2rust D3D_TESSELLATOR_DOMAIN_TRI          = d3d::TessellatorDomain::Tri
//#cpp2rust D3D_TESSELLATOR_DOMAIN_QUAD         = d3d::TessellatorDomain::Quad

//#cpp2rust D3D11_TESSELLATOR_DOMAIN_UNDEFINED  = d3d::TessellatorDomain::Undefined
//#cpp2rust D3D11_TESSELLATOR_DOMAIN_ISOLINE    = d3d::TessellatorDomain::IsoLine
//#cpp2rust D3D11_TESSELLATOR_DOMAIN_TRI        = d3d::TessellatorDomain::Tri
//#cpp2rust D3D11_TESSELLATOR_DOMAIN_QUAD       = d3d::TessellatorDomain::Quad
