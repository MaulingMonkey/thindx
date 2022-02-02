#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_register_component_type)\]
/// D3D_REGISTER_COMPONENT_TYPE / D3D_COMPONENT_TYPE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct RegisterComponentType(D3D_REGISTER_COMPONENT_TYPE);
#[doc(hidden)] pub use RegisterComponentType as RegisterComponent;

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { RegisterComponent => D3D_REGISTER_COMPONENT_TYPE; default: Unknown == 0; Unknown, UInt32, SInt32, Float32 }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl RegisterComponent { // These are enum-like
    pub const Unknown   : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_UNKNOWN); // 0
    pub const UInt32    : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_UINT32);
    pub const SInt32    : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_SINT32);
    pub const Float32   : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_FLOAT32);
}

//#cpp2rust D3D_REGISTER_COMPONENT_TYPE         = d3d::RegisterComponentType

//#cpp2rust D3D_REGISTER_COMPONENT_UNKNOWN      = d3d::RegisterComponent::Unknown
//#cpp2rust D3D_REGISTER_COMPONENT_UINT32       = d3d::RegisterComponent::UInt32
//#cpp2rust D3D_REGISTER_COMPONENT_SINT32       = d3d::RegisterComponent::SInt32
//#cpp2rust D3D_REGISTER_COMPONENT_FLOAT32      = d3d::RegisterComponent::Float32

//#cpp2rust D3D10_REGISTER_COMPONENT_UNKNOWN    = d3d::RegisterComponent::Unknown
//#cpp2rust D3D10_REGISTER_COMPONENT_UINT32     = d3d::RegisterComponent::UInt32
//#cpp2rust D3D10_REGISTER_COMPONENT_SINT32     = d3d::RegisterComponent::SInt32
//#cpp2rust D3D10_REGISTER_COMPONENT_FLOAT32    = d3d::RegisterComponent::Float32
