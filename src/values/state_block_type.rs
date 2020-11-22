#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstateblocktype)\]
/// D3DSTATEBLOCKTYPE
///
/// Predefined sets of pipeline state used by state blocks (see [State Blocks Save and Restore State (Direct3D 9)]).
///
/// [State Blocks Save and Restore State (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct StateBlockType(D3DSTATEBLOCKTYPE);

impl StateBlockType {
    /// Convert a raw [D3DSTATEBLOCKTYPE] value into a [StateBlockType].  This is *probably* safe... probably....
    ///
    /// [D3DSTATEBLOCKTYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstateblocktype
    pub const fn from_unchecked(stateblocktype: D3DSTATEBLOCKTYPE) -> Self { Self(stateblocktype) }

    /// Convert a [StateBlockType] into a raw [D3DSTATEBLOCKTYPE].
    ///
    /// [D3DSTATEBLOCKTYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstateblocktype
    pub const fn into(self) -> D3DSTATEBLOCKTYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl StateBlockType {
    /// Capture the current [device state](https://docs.microsoft.com/en-us/windows/win32/direct3d9/saving-all-device-states-with-a-stateblock).
    pub const All           : StateBlockType = StateBlockType(D3DSBT_ALL);

    /// Capture the current [pixel state](https://docs.microsoft.com/en-us/windows/win32/direct3d9/saving-pixel-states-with-a-stateblock).
    pub const PixelState    : StateBlockType = StateBlockType(D3DSBT_PIXELSTATE);

    /// Capture the current [vertex state](https://docs.microsoft.com/en-us/windows/win32/direct3d9/saving-vertex-states-with-a-stateblock).
    pub const VertexState   : StateBlockType = StateBlockType(D3DSBT_VERTEXSTATE);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for StateBlockType {
    fn default() -> Self { StateBlockType::All }
}

impl Debug for StateBlockType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            StateBlockType::All         => write!(f, "StateBlockType::All"),
            StateBlockType::PixelState  => write!(f, "StateBlockType::PixelState"),
            StateBlockType::VertexState => write!(f, "StateBlockType::VertexState"),
            other                       => write!(f, "StateBlockType({})", other.0),
        }
    }
}

impl From<StateBlockType> for D3DSTATEBLOCKTYPE {
    fn from(value: StateBlockType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DSTATEBLOCKTYPE> for StateBlockType {
    fn from(value: D3DSTATEBLOCKTYPE) -> Self { Self(value) }
}
