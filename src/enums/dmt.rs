#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddebugmonitortokens)\]
/// D3DDEBUGMONITORTOKENS
///
/// Defines the debug monitor tokens.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DMT(D3DDEBUGMONITORTOKENS);
pub type DebugMonitorTokens = DMT;

enumish! { DMT => D3DDEBUGMONITORTOKENS; Enable, Disable }

#[allow(non_upper_case_globals)] impl DMT { // These are enum-like
    pub const Enable        : DMT = DMT(D3DDMT_ENABLE); // 0
    pub const Disable       : DMT = DMT(D3DDMT_DISABLE); // 1
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for DMT {
    fn default() -> Self { DMT::Disable } // 1
}
