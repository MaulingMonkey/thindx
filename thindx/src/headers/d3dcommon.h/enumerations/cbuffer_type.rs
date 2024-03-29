#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_cbuffer_type)\]
/// D3D_CBUFFER_TYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct CBufferType(D3D_CBUFFER_TYPE);
#[doc(hidden)] pub use CBufferType as CT;

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { CT => D3D_CBUFFER_TYPE; default: CBuffer == 0; CBuffer, TBuffer, InterfacePointers, ResourceBindInfo }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl CT { // These are enum-like
    pub const CBuffer               : CT = CT(D3D_CT_CBUFFER); // 0
    pub const TBuffer               : CT = CT(D3D_CT_TBUFFER);
    pub const InterfacePointers     : CT = CT(D3D_CT_INTERFACE_POINTERS);
    pub const ResourceBindInfo      : CT = CT(D3D_CT_RESOURCE_BIND_INFO);
}

//#cpp2rust D3D_CBUFFER_TYPE            = d3d::CBufferType

//#cpp2rust D3D_CT_CBUFFER              = d3d::CT::CBuffer
//#cpp2rust D3D_CT_TBUFFER              = d3d::CT::TBuffer
//#cpp2rust D3D_CT_INTERFACE_POINTERS   = d3d::CT::InterfacePointers
//#cpp2rust D3D_CT_RESOURCE_BIND_INFO   = d3d::CT::ResourceBindInfo

//#cpp2rust D3D10_CT_CBUFFER            = d3d::CT::CBuffer
//#cpp2rust D3D10_CT_TBUFFER            = d3d::CT::TBuffer
//#cpp2rust D3D10_CT_INTERFACE_POINTERS = d3d::CT::InterfacePointers
//#cpp2rust D3D10_CT_RESOURCE_BIND_INFO = d3d::CT::ResourceBindInfo

//#cpp2rust D3D11_CT_CBUFFER            = d3d::CT::CBuffer
//#cpp2rust D3D11_CT_TBUFFER            = d3d::CT::TBuffer
//#cpp2rust D3D11_CT_INTERFACE_POINTERS = d3d::CT::InterfacePointers
//#cpp2rust D3D11_CT_RESOURCE_BIND_INFO = d3d::CT::ResourceBindInfo
