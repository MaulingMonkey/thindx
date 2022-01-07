#[doc(no_inline)] pub use crate::misc::*;
#[doc(no_inline)] pub use crate::win32::*;

#[doc(no_inline)] pub use thindx::*;
#[doc(no_inline)] pub use thindx::d3d::*;
#[doc(no_inline)] pub use thindx::d3d9::*;

use winapi::shared::d3d9caps::*;
use winapi::shared::d3d9types::*;

// XXX: temporary?

#[doc(no_inline)] pub use winapi::shared::d3d9caps::{
    D3DCAPS9,
};
#[doc(no_inline)] pub use winapi::shared::d3d9types::{
    D3DDISPLAYMODE,
    D3DDISPLAYMODEEX,
    D3DPRESENT_PARAMETERS,
};
#[doc(no_inline)] pub use winapi::shared::windef::{
    HWND,
    HMONITOR,
};
pub type AdapterIndex = u32;
pub const D3DADAPTER_DEFAULT : AdapterIndex = 0;



// Direct3D{,Ex}

#[cfg(feature = "9ex")]
pub fn d3dex_test() -> Direct3DEx { unsafe { Direct3DEx::create(SdkVersion::default()).unwrap() } }
pub fn d3d_test()   -> Direct3D   { unsafe { Direct3D  ::create(SdkVersion::default()).unwrap() } }



// SafeDevice

pub fn safe_device_test() -> SafeDevice { SafeDevice::new(device_test()).unwrap() }
pub fn safe_device_pure() -> SafeDevice { SafeDevice::new(device_pure()).unwrap() }
pub fn safe_device_test_pp(two: bool, ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS, &mut Create)) -> Result<SafeDevice, MethodError> { Ok(SafeDevice::new(device_test_pp(two, ppf)?)?) }



// Device

pub fn device_test() -> Device { device_test_pp(false, |_,_|{}).unwrap() }
pub fn device_pure() -> Device { device_test_pp(false, |_,c| *c |= Create::PureDevice).unwrap() }
pub fn device_test2() -> [Device; 2] {[
    device_test_pp(false, |_,_|{}).unwrap(),
    device_test_pp(true,  |_,_|{}).unwrap(),
]}
pub fn device_pure2() -> [Device; 2] {[
    device_test_pp(false, |_,c| *c |= Create::PureDevice).unwrap(),
    device_test_pp(true,  |_,c| *c |= Create::PureDevice).unwrap(),
]}

pub fn device_test_pp(two: bool, ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS, &mut Create)) -> Result<Device, MethodError> {
    let mut behavior = /* Create::DisablePrintScreen | */ Create::FpuPreserve | Create::HardwareVertexProcessing | Create::NoWindowChanges;
    let mut pp = D3DPRESENT_PARAMETERS {
        Windowed:               true.into(),
        hDeviceWindow:          test_window(two),
        SwapEffect:             D3DSWAPEFFECT_DISCARD,
        PresentationInterval:   D3DPRESENT_INTERVAL_IMMEDIATE,
        .. unsafe { std::mem::zeroed() }
    };

    ppf(&mut pp, &mut behavior);
    let d3d = d3d_test();
    unsafe { d3d.create_device(0, DevType::HAL, std::ptr::null_mut(), behavior, &mut pp) }
}



// DeviceEx

#[cfg(feature = "9ex")] pub fn device_ex_test() -> DeviceEx { device_ex_test_pp(false, |_, _|{}).unwrap() }
#[cfg(feature = "9ex")] pub fn device_ex_pure() -> DeviceEx { device_ex_test_pp(false, |_,c| *c |= Create::PureDevice).unwrap() }

#[cfg(feature = "9ex")] pub fn device_ex_test2() -> [DeviceEx; 2] {[
    device_ex_test_pp(false, |_,_|{}).unwrap(),
    device_ex_test_pp(true,  |_,_|{}).unwrap(),
]}

#[cfg(feature = "9ex")] pub fn device_ex_pure2() -> [DeviceEx; 2] {[
    device_ex_test_pp(false, |_,c| *c |= Create::PureDevice).unwrap(),
    device_ex_test_pp(true,  |_,c| *c |= Create::PureDevice).unwrap(),
]}

#[cfg(feature = "9ex")] pub fn device_ex_test_pp(two: bool, ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS, &mut Create)) -> Result<DeviceEx, MethodError> {
    let mut behavior = Create::DisablePrintScreen | Create::FpuPreserve | Create::HardwareVertexProcessing | Create::NoWindowChanges;
    let mut pp = D3DPRESENT_PARAMETERS {
        Windowed:               true.into(),
        hDeviceWindow:          test_window(two),
        SwapEffect:             D3DSWAPEFFECT_DISCARD,
        PresentationInterval:   D3DPRESENT_INTERVAL_IMMEDIATE,
        .. unsafe { std::mem::zeroed() }
    };

    ppf(&mut pp, &mut behavior);
    let d3d = d3dex_test();
    unsafe { d3d.create_device_ex(0, DevType::HAL, std::ptr::null_mut(), behavior, &mut pp, &mut []) }
}



pub struct Invalid;
impl From<Invalid> for DevType         { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
impl From<Invalid> for Format          { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
impl From<Invalid> for FVF             { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
impl From<Invalid> for MultiSample     { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
impl From<Invalid> for Pool            { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
impl From<Invalid> for ResourceType    { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
impl From<Invalid> for Usage           { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
