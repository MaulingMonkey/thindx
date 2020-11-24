#![allow(dead_code)] // TODO: remove

use crate::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dstateblock9)\]
/// Used to [capture/save and restore](https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state)
/// changes to [Device] state.
///
/// ### See Also
///
/// *   [Device::begin_state_block]
/// *   [Device::create_state_block]
/// *   [Device::end_state_block]
#[derive(Clone)] #[repr(transparent)]
pub struct StateBlock(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DStateBlock9>);



/// # StateBlocks
/// Create/Capture/Replay Direct3D states via [StateBlock]s
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginstateblock)\]
    /// IDirect3DDevice9::BeginStateBlock
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device was already within a state block
    /// *   `Ok(())`                otherwise
    pub fn begin_state_block(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.0.BeginStateBlock() };
        MethodError::check("IDirect3DDevice9::BeginStateBlock", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createstateblock)\]
    /// IDirect3DDevice9::CreateStateBlock
    ///
    /// Creates a new state block that contains the values for all device states, vertex-related states, or pixel-related states.
    ///
    /// Vertex-related device states typically refer to those states that affect how the system processes vertices.
    /// Pixel-related states generally refer to device states that affect how the system processes pixel or depth-buffer data during rasterization.
    /// Some states are contained in both groups.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok([StateBlock])
    pub(crate) fn create_state_block(&self, type_: StateBlockType) -> Result<StateBlock, MethodError> {
        let mut sb = null_mut();
        let hr = unsafe { self.0.CreateStateBlock(type_.into(), &mut sb) };
        MethodError::check("IDirect3DDevice9::CreateStateBlock", hr)?;
        Ok(unsafe { StateBlock::from_raw(sb) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endstateblock)\]
    /// IDirect3DDevice9::EndStateBlock
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device wasn't within a state block
    /// *   `Ok(state_block)`       if a state block was successfully captured
    pub fn end_state_block(&self) -> Result<StateBlock, MethodError> {
        let mut sb = null_mut();
        let hr = unsafe { self.0.EndStateBlock(&mut sb) };
        MethodError::check("IDirect3DDevice9::EndStateBlock", hr)?;
        Ok(unsafe { StateBlock::from_raw(sb) })
    }

    // TODO: fn state_block_scope(&self) with sane drop behavior?
    // TODO: examples
}

#[test] fn begin_end_state_block() {
    let device = Device::test();
    assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().err()); // not in a state block

    device.begin_state_block().unwrap();
    device.end_state_block().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().err()); // not in a state block

    device.begin_state_block().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.begin_state_block().err()); // cannot nest state blocks
    device.end_state_block().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().err());
}

// TODO: test explicit state capturing



impl StateBlock {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-apply)\]
    /// IDirect3DStateBlock9::Apply
    ///
    /// Apply the state block to the current device state.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    pub fn apply(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.0.Apply() };
        MethodError::check("IDirect3DStateBlock9::Apply", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-capture)\]
    /// IDirect3DStateBlock9::Capture
    ///
    /// Capture the current value of states that are included in a stateblock.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    pub fn capture(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.0.Capture() };
        MethodError::check("IDirect3DStateBlock9::Capture", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-getdevice)\]
    /// IDirect3DStateBlock9::GetDevice
    ///
    /// Gets the device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   (Pure device?)
    /// *   Ok([Device])
    pub fn get_device(&self) -> Result<Device, MethodError> {
        let mut device = null_mut();
        let hr = unsafe { self.0.GetDevice(&mut device) };
        MethodError::check("IDirect3DStateBlock9::GetDevice", hr)?;
        Ok(unsafe { Device::from_raw(device) })
    }
}
