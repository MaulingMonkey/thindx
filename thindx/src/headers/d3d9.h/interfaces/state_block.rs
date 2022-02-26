#![allow(dead_code)] // TODO: remove

use winapi::shared::d3d9::IDirect3DStateBlock9;
use winapi::um::unknwnbase::IUnknown;

use crate::*;
use crate::d3d9::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dstateblock9)\]
/// Used to [capture/save and restore](https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state)
/// changes to [Device] state.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::begin_state_block]
/// *   [IDirect3DDevice9Ext::create_state_block]
/// *   [IDirect3DDevice9Ext::end_state_block]
#[derive(Clone)] #[repr(transparent)]
pub struct StateBlock(pub(crate) mcom::Rc<IDirect3DStateBlock9>);

unsafe impl AsSafe<IUnknown             > for StateBlock { fn as_safe(&self) -> &IUnknown               { &**self.0 } }
unsafe impl AsSafe<IDirect3DStateBlock9 > for StateBlock { fn as_safe(&self) -> &IDirect3DStateBlock9   { &*self.0 } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dstateblock9)\]
/// IDirect3DStateBlock9 extension methods
///
/// ### Methods
/// | thindx                                                        | docs.microsoft.com    | Description |
/// | ------------------------------------------------------------- | --------------------- | ----------- |
/// | [apply](Self::apply)                                          | [Apply]               | Apply the state block to the current device state.
/// | [capture](Self::capture)                                      | [Capture]             | Capture the current value of states that are included in a stateblock.
/// | [get_device](Self::get_device)                                | [GetDevice]           | Gets the device.
///
/// [Apply]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-apply
/// [Capture]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-capture
/// [GetDevice]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-getdevice
///
pub trait IDirect3DStateBlock9Ext : AsSafe<IDirect3DStateBlock9> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-apply)\]
    /// IDirect3DStateBlock9::Apply
    ///
    /// Apply the state block to the current device state.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    fn apply(&self) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DStateBlock9Ext::apply => IDirect3DStateBlock9::Apply);
        fn_check_hr!(unsafe { self.as_winapi().Apply() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-capture)\]
    /// IDirect3DStateBlock9::Capture
    ///
    /// Capture the current value of states that are included in a stateblock.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    fn capture(&self) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DStateBlock9Ext::capture => IDirect3DStateBlock9::Capture);
        fn_check_hr!(unsafe { self.as_winapi().Capture() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dstateblock9-getdevice)\]
    /// IDirect3DStateBlock9::GetDevice
    ///
    /// Gets the device.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   (Pure device?)
    /// *   Ok([Device])
    fn get_device(&self) -> Result<Device, Error> {
        fn_context!(d3d9::IDirect3DStateBlock9Ext::get_device => IDirect3DStateBlock9::GetDevice);
        let mut device = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetDevice(&mut device) })?;
        Ok(unsafe { Device::from_raw(device) })
    }
}

impl<T: AsSafe<IDirect3DStateBlock9>> IDirect3DStateBlock9Ext for T {}



#[test] fn begin_end_state_block() {
    use dev::d3d9::*;

    let device = device_test();
    assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().map(|_|())); // not in a state block

    device.begin_state_block().unwrap();
    device.end_state_block().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().map(|_|())); // not in a state block

    device.begin_state_block().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.begin_state_block()); // cannot nest state blocks
    device.end_state_block().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().map(|_|()));
}

// TODO: test explicit state capturing



//#cpp2rust IDirect3DStateBlock9                    = d3d9::StateBlock
//#cpp2rust IDirect3DStateBlock9                    = d3d9::IDirect3DStateBlock9Ext
