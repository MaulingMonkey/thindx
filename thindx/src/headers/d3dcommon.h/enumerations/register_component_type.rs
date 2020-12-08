#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_register_component_type)\]
/// D3D_REGISTER_COMPONENT_TYPE / D3D_COMPONENT_TYPE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RegisterComponentType(D3D_REGISTER_COMPONENT_TYPE);
#[doc(hidden)] pub use RegisterComponentType as RegisterComponent;

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { RegisterComponent => D3D_REGISTER_COMPONENT_TYPE; Unknown, UInt32, SInt32, Float32 }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl RegisterComponent { // These are enum-like
    pub const Unknown   : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_UNKNOWN);
    pub const UInt32    : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_UINT32);
    pub const SInt32    : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_SINT32);
    pub const Float32   : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_FLOAT32);
}

#[doc(hidden)] impl RegisterComponent { // Ctrl+C Ctrl+V support
    pub const UNKNOWN   : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_UNKNOWN);
    pub const UINT32    : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_UINT32);
    pub const SINT32    : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_SINT32);
    pub const FLOAT32   : RegisterComponent = RegisterComponent(D3D_REGISTER_COMPONENT_FLOAT32);
}

impl Default for RegisterComponent {
    fn default() -> Self { RegisterComponent(0) }
}
