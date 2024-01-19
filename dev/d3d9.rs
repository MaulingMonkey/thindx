#[doc(no_inline)] pub use crate::misc::*;
#[doc(no_inline)] pub use crate::win32::*;

#[doc(no_inline)] pub use thindx::{*, ErrorKind};
#[doc(no_inline)] pub use thindx::Error;
#[doc(no_inline)] pub use thindx::d3d::{*, Cursor};
#[doc(no_inline)] pub use thindx::d3d9::*;

use std::fs::*;
use std::io::*;
use std::mem::swap;
use std::path::*;
use std::result::Result;



// XXX: temporary?

#[doc(no_inline)] pub use winapi::shared::windef::{
    HWND,
    HMONITOR,
};
pub type AdapterIndex = u32;
pub const D3DADAPTER_DEFAULT : AdapterIndex = 0;



// Direct3D{,Ex}

#[cfg(feature = "9ex")]
pub fn d3dex_test() -> Direct3DEx { unsafe { Direct3DEx::create_ex(SdkVersion::default()).unwrap() } }
pub fn d3d_test()   -> Direct3D   { unsafe { Direct3D  ::create(SdkVersion::default()).unwrap() } }



// SafeDevice

pub fn safe_device_test() -> SafeDevice { SafeDevice::new(device_test()).unwrap() }
pub fn safe_device_pure() -> SafeDevice { SafeDevice::new(device_pure()).unwrap() }
pub fn safe_device_test_pp(two: bool, ppf: impl FnOnce(&mut PresentParameters, &mut Create)) -> Result<SafeDevice, Error> { SafeDevice::new(device_test_pp(two, ppf)?) }



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

pub fn device_test_pp(two: bool, ppf: impl FnOnce(&mut PresentParameters, &mut Create)) -> Result<Device, Error> {
    let mut behavior = /* Create::DisablePrintScreen | */ Create::FpuPreserve | Create::HardwareVertexProcessing | Create::NoWindowChanges;
    let mut pp = PresentParameters {
        windowed:               true.into(),
        device_window:          Some(test_window(two)),
        swap_effect:            SwapEffect::Discard,
        presentation_interval:  Present::IntervalImmediate,
        .. unsafe { std::mem::zeroed() }
    };

    ppf(&mut pp, &mut behavior);
    let d3d = d3d_test();
    unsafe { d3d.create_device(0, DevType::HAL, None, behavior, &mut pp) }
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

#[cfg(feature = "9ex")] pub fn device_ex_test_pp(two: bool, ppf: impl FnOnce(&mut PresentParameters, &mut Create)) -> Result<DeviceEx, Error> {
    let mut behavior = Create::DisablePrintScreen | Create::FpuPreserve | Create::HardwareVertexProcessing | Create::NoWindowChanges;
    let mut pp = PresentParameters {
        windowed:               true.into(),
        device_window:          Some(test_window(two)),
        swap_effect:            SwapEffect::Discard,
        presentation_interval:  Present::IntervalImmediate,
        .. PresentParameters::zeroed()
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



pub fn hide_for_docs_gen() -> bool { std::env::var_os("THINDX_DOCS_EXAMPLE_SCREENSHOT_PATH").is_some() }

pub fn screenshot_rt0_for_docs_gen(device: &Device) {
    if let Some(screenshot_path) = std::env::var_os("THINDX_DOCS_EXAMPLE_SCREENSHOT_PATH").map(PathBuf::from) {
        let rt      = device.get_render_target(0).unwrap().unwrap();

        let desc    = rt.get_desc().unwrap();
        let width   = desc.width as usize;
        let height  = desc.height as usize;
        let surface = device.create_offscreen_plain_surface(desc.width, desc.height, Format::X8R8G8B8, Pool::SystemMem, ()).unwrap();
        let bpp     = 4;

        device.get_render_target_data(&rt, &surface).unwrap();

        let mut data = Vec::new();
        data.resize(bpp * width * height, 0);
        unsafe {
            let lock = surface.lock_rect_unchecked(.., Lock::None).unwrap();
            for y in 0..height {
                let src = (lock.pBits as *mut u8).add(y * lock.Pitch as usize);
                let dst = data[y * width * bpp ..].as_mut_ptr();
                std::ptr::copy(src, dst.cast(), bpp * width);
            }
            surface.unlock_rect().unwrap();
        }

        let mut pending = &mut data[..];
        while let [r, _g, b, x, rest @ ..] = pending {
            swap(r, b); // BGR. => RGB. (fix d3d9 format => png format)
            *x = !0;    // ...X => ...A (make screenshot opaque regardless of back buffer alpha/x channel)
            pending = rest;
        }
        for i in 0 .. width*height { data[4*i + 3] = !0; }

        let file = BufWriter::new(File::create(screenshot_path).unwrap());
        let mut encoder = png::Encoder::new(file, desc.width, desc.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        // set_trns ?
        // set_source_gamma ?
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&data[..]).unwrap();
        drop(writer);
        std::process::exit(0);
    }
}
