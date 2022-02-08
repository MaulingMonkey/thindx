//! Direct3D related types and APIs used across multiple Direct3D versions.

pub use crate::d3dcommon_h::*;
pub use crate::d3dcompiler_h::*;
pub use crate::d3d9_h::{Cursor, perf};
pub use crate::d3d9caps_h::*;
pub use crate::d3d9types_h::*; // TODO: exclude d3d9::ShaderVersion (conflicts with d3d11::ShaderVersion)



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drange)\]
/// D3DRANGE
pub type Range = std::ops::Range<u32>; // TODO: make a Pod/Zeroable/??? struct ala other d3d9 structs?

//#cpp2rust D3DRANGE = d3d::Range
