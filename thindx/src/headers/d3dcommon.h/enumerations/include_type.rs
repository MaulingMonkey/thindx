#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_include_type)\]
/// D3D_INCLUDE_TYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct IncludeType(D3D_INCLUDE_TYPE);
#[doc(hidden)] pub use IncludeType as Include;

enumish! { Include => D3D_INCLUDE_TYPE; Local, System }

#[allow(non_upper_case_globals)] impl Include { // These are enum-like
    /// A "local" `#include "..."` style include
    pub const Local     : Include = Include(D3D_INCLUDE_LOCAL);

    /// A "system" `#include <...>` style include
    pub const System    : Include = Include(D3D_INCLUDE_SYSTEM);
}

#[doc(hidden)] impl Include { // Ctrl+C Ctrl+V support
    pub const LOCAL     : Include = Include(D3D_INCLUDE_LOCAL);
    pub const SYSTEM    : Include = Include(D3D_INCLUDE_SYSTEM);
}

impl Default for Include {
    fn default() -> Self { Include(0) }
}
