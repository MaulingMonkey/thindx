#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_name)\]
/// D3D_NAME
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Name(D3D_NAME);

enumish! {
    Name => D3D_NAME;
}

#[allow(non_upper_case_globals)] impl Name { // These are enum-like
    // TODO
}

#[doc(hidden)] impl Name { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for Name {
    fn default() -> Self { Name(0) }
}
