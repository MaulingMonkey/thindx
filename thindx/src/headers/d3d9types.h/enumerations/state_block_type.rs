#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstateblocktype)\]
/// D3DSTATEBLOCKTYPE
///
/// Predefined sets of pipeline state used by state blocks (see [State Blocks Save and Restore State (Direct3D 9)]).
///
/// [State Blocks Save and Restore State (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct StateBlockType(D3DSTATEBLOCKTYPE);
pub use StateBlockType as SBT;

enumish! { SBT => D3DSTATEBLOCKTYPE; All, PixelState, VertexState }

#[allow(non_upper_case_globals)] impl StateBlockType { // These are enum-like
    /// Capture the current [device state](https://docs.microsoft.com/en-us/windows/win32/direct3d9/saving-all-device-states-with-a-stateblock).
    pub const All           : StateBlockType = StateBlockType(D3DSBT_ALL);

    /// Capture the current [pixel state](https://docs.microsoft.com/en-us/windows/win32/direct3d9/saving-pixel-states-with-a-stateblock).
    pub const PixelState    : StateBlockType = StateBlockType(D3DSBT_PIXELSTATE);

    /// Capture the current [vertex state](https://docs.microsoft.com/en-us/windows/win32/direct3d9/saving-vertex-states-with-a-stateblock).
    pub const VertexState   : StateBlockType = StateBlockType(D3DSBT_VERTEXSTATE);
}

impl StateBlockType {
    pub const fn zeroed() -> Self { Self(0) }
}

//#cpp2rust D3DSTATEBLOCKTYPE       = d3d::StateBlockType
//#cpp2rust D3DSBT_ALL              = d3d::SBT::All
//#cpp2rust D3DSBT_PIXELSTATE       = d3d::SBT::PixelState
//#cpp2rust D3DSBT_VERTEXSTATE      = d3d::SBT::VertexState
