#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive)\]
/// D3D_PRIMITIVE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Primitive(D3D_PRIMITIVE);

enumish! {
    Primitive => D3D_PRIMITIVE;
}

#[allow(non_upper_case_globals)] impl Primitive { // These are enum-like
    // TODO
}

#[doc(hidden)] impl Primitive { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for Primitive {
    fn default() -> Self { Primitive(0) }
}
