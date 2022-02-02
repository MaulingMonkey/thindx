#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_include_type)\]
/// D3D_INCLUDE_TYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct IncludeType(D3D_INCLUDE_TYPE);
#[doc(hidden)] pub use IncludeType as Include;

enumish! { Include => D3D_INCLUDE_TYPE; default: Local == 0; Local, System }

#[allow(non_upper_case_globals)] impl Include { // These are enum-like
    /// A "local" `#include "..."` style include
    pub const Local     : Include = Include(D3D_INCLUDE_LOCAL);

    /// A "system" `#include <...>` style include
    pub const System    : Include = Include(D3D_INCLUDE_SYSTEM);
}

//#cpp2rust D3D_INCLUDE_TYPE        = d3d::IncludeType

//#cpp2rust D3D_INCLUDE_LOCAL       = d3d::Include::Local
//#cpp2rust D3D_INCLUDE_SYSTEM      = d3d::Include::System

//#cpp2rust D3D10_INCLUDE_LOCAL     = d3d::Include::Local
//#cpp2rust D3D10_INCLUDE_SYSTEM    = d3d::Include::System
