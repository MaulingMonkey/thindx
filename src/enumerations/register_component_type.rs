#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_register_component_type)\]
/// D3D_REGISTER_COMPONENT_TYPE / D3D_COMPONENT_TYPE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RegisterComponentType(D3D_REGISTER_COMPONENT_TYPE);
#[doc(hidden)] pub use RegisterComponentType as RegisterComponent;

enumish! {
    RegisterComponent => D3D_REGISTER_COMPONENT_TYPE;
}

#[allow(non_upper_case_globals)] impl RegisterComponent { // These are enum-like
    // TODO
}

#[doc(hidden)] impl RegisterComponent { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for RegisterComponent {
    fn default() -> Self { RegisterComponent(0) }
}
