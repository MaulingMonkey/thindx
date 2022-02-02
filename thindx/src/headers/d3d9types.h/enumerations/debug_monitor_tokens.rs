#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddebugmonitortokens)\]
/// D3DDEBUGMONITORTOKENS
///
/// Defines the debug monitor tokens.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct DebugMonitorTokens(D3DDEBUGMONITORTOKENS);
pub use DebugMonitorTokens as DMT;

enumish! { DMT => D3DDEBUGMONITORTOKENS; Enable, Disable }

#[allow(non_upper_case_globals)] impl DMT { // These are enum-like
    pub const Enable        : DMT = DMT(D3DDMT_ENABLE); // 0
    pub const Disable       : DMT = DMT(D3DDMT_DISABLE); // 1
}

impl DMT {
    pub const fn zeroed() -> Self { Self(0) }
}

//#cpp2rust D3DDEBUGMONITORTOKENS   = d3d::DebugMonitorTokens
//#cpp2rust D3DDMT_ENABLE           = d3d::DMT::Enable
//#cpp2rust D3DDMT_DISABLE          = d3d::DMT::Disable
