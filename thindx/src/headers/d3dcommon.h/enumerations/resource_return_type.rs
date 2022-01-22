#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_resource_return_type)\]
/// D3D_RESOURCE_RETURN_TYPE / D3D_RETURN_TYPE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ResourceReturnType(D3D_RESOURCE_RETURN_TYPE);
#[doc(hidden)] pub use ResourceReturnType as ReturnType;

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { ReturnType => D3D_RESOURCE_RETURN_TYPE; UNorm, SNorm, SInt, UInt, Float, Mixed, Double, Continued }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl ReturnType { // These are enum-like
    pub const UNorm     : ReturnType = ReturnType(D3D_RETURN_TYPE_UNORM); // 1
    pub const SNorm     : ReturnType = ReturnType(D3D_RETURN_TYPE_SNORM);
    pub const SInt      : ReturnType = ReturnType(D3D_RETURN_TYPE_SINT);
    pub const UInt      : ReturnType = ReturnType(D3D_RETURN_TYPE_UINT);
    pub const Float     : ReturnType = ReturnType(D3D_RETURN_TYPE_FLOAT);
    pub const Mixed     : ReturnType = ReturnType(D3D_RETURN_TYPE_MIXED);
    pub const Double    : ReturnType = ReturnType(D3D_RETURN_TYPE_DOUBLE);
    pub const Continued : ReturnType = ReturnType(D3D_RETURN_TYPE_CONTINUED);
}

#[doc(hidden)] impl ReturnType { // Ctrl+C Ctrl+V support
    pub const UNORM     : ReturnType = ReturnType(D3D_RETURN_TYPE_UNORM); // 1
    pub const SNORM     : ReturnType = ReturnType(D3D_RETURN_TYPE_SNORM);
    pub const SINT      : ReturnType = ReturnType(D3D_RETURN_TYPE_SINT);
    pub const UINT      : ReturnType = ReturnType(D3D_RETURN_TYPE_UINT);
    pub const FLOAT     : ReturnType = ReturnType(D3D_RETURN_TYPE_FLOAT);
    pub const MIXED     : ReturnType = ReturnType(D3D_RETURN_TYPE_MIXED);
    pub const DOUBLE    : ReturnType = ReturnType(D3D_RETURN_TYPE_DOUBLE);
    pub const CONTINUED : ReturnType = ReturnType(D3D_RETURN_TYPE_CONTINUED);
}

impl Default for ReturnType {
    fn default() -> Self { ReturnType(0) }
}

//#cpp2rust D3D_RESOURCE_RETURN_TYPE    = d3d::ResourceReturnType

//#cpp2rust D3D_RETURN_TYPE_UNORM       = d3d::ReturnType::UNorm
//#cpp2rust D3D_RETURN_TYPE_SNORM       = d3d::ReturnType::SNorm
//#cpp2rust D3D_RETURN_TYPE_SINT        = d3d::ReturnType::SInt
//#cpp2rust D3D_RETURN_TYPE_UINT        = d3d::ReturnType::UInt
//#cpp2rust D3D_RETURN_TYPE_FLOAT       = d3d::ReturnType::Float
//#cpp2rust D3D_RETURN_TYPE_MIXED       = d3d::ReturnType::Mixed
//#cpp2rust D3D_RETURN_TYPE_DOUBLE      = d3d::ReturnType::Double
//#cpp2rust D3D_RETURN_TYPE_CONTINUED   = d3d::ReturnType::Continued

//#cpp2rust D3D10_RETURN_TYPE_UNORM     = d3d::ReturnType::UNorm
//#cpp2rust D3D10_RETURN_TYPE_SNORM     = d3d::ReturnType::SNorm
//#cpp2rust D3D10_RETURN_TYPE_SINT      = d3d::ReturnType::SInt
//#cpp2rust D3D10_RETURN_TYPE_UINT      = d3d::ReturnType::UInt
//#cpp2rust D3D10_RETURN_TYPE_FLOAT     = d3d::ReturnType::Float
//#cpp2rust D3D10_RETURN_TYPE_MIXED     = d3d::ReturnType::Mixed

//#cpp2rust D3D11_RETURN_TYPE_UNORM     = d3d::ReturnType::UNorm
//#cpp2rust D3D11_RETURN_TYPE_SNORM     = d3d::ReturnType::SNorm
//#cpp2rust D3D11_RETURN_TYPE_SINT      = d3d::ReturnType::SInt
//#cpp2rust D3D11_RETURN_TYPE_UINT      = d3d::ReturnType::UInt
//#cpp2rust D3D11_RETURN_TYPE_FLOAT     = d3d::ReturnType::Float
//#cpp2rust D3D11_RETURN_TYPE_MIXED     = d3d::ReturnType::Mixed
//#cpp2rust D3D11_RETURN_TYPE_DOUBLE    = d3d::ReturnType::Double
//#cpp2rust D3D11_RETURN_TYPE_CONTINUED = d3d::ReturnType::Continued
