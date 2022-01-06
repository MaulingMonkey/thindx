#![allow(dead_code)] // TODO: remove

use crate::*;
use crate::d3d9::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dstateblock9)\]
/// Used to [capture/save and restore](https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state)
/// changes to [Device] state.
///
/// ### See Also
///
/// *   [IDirect3DDevice9Ext::begin_state_block]
/// *   [IDirect3DDevice9Ext::create_state_block]
/// *   [IDirect3DDevice9Ext::end_state_block]
#[derive(Clone)] #[repr(transparent)]
pub struct StateBlock(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DStateBlock9>);

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



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dstateblock9)\]
/// IDirect3DStateBlock9 extension methods
///
/// ### Methods
///
/// | thin3d9                                                       | docs.microsoft.com    | Description |
/// | ------------------------------------------------------------- | --------------------- | ----------- |
/// | [apply](Self::apply)                                          | [Apply]               | Apply the state block to the current device state.
/// | [capture](Self::capture)                                      | [Capture]             | Capture the current value of states that are included in a stateblock.
/// | [get_device](Self::get_device)                                | [GetDevice]           | Gets the device.
///
/// [Apply]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-apply
/// [Capture]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-capture
/// [GetDevice]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-getdevice
///
pub trait IDirect3DStateBlock9Ext : private::Sealed {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-apply)\]
    /// IDirect3DStateBlock9::Apply
    ///
    /// Apply the state block to the current device state.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    fn apply(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().Apply() };
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
    fn capture(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().Capture() };
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
    fn get_device(&self) -> Result<Device, MethodError> {
        let mut device = null_mut();
        let hr = unsafe { self.as_winapi().GetDevice(&mut device) };
        MethodError::check("IDirect3DStateBlock9::GetDevice", hr)?;
        Ok(unsafe { Device::from_raw(device) })
    }
}

impl<T: private::Sealed> IDirect3DStateBlock9Ext for T {}

mod private {
    use winapi::shared::d3d9::IDirect3DStateBlock9;
    pub unsafe trait Sealed { fn as_winapi(&self) -> &IDirect3DStateBlock9; }
    unsafe impl Sealed for mcom::Rc<IDirect3DStateBlock9>   { fn as_winapi(&self) -> &IDirect3DStateBlock9 { &**self } }
    unsafe impl Sealed for super::StateBlock                { fn as_winapi(&self) -> &IDirect3DStateBlock9 { &*self.0 } }
}
