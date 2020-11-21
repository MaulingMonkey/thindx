#![cfg(feature = "9ex")]

use crate::*;

use winapi::shared::d3d9::Direct3DCreate9Ex;

use std::ptr::null_mut;



impl Direct3DEx {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-direct3dcreate9ex)\]
    /// Direct3DCreate9Ex
    ///
    /// ### Safety
    ///
    /// While this individual call should be 100% safe/sound, there is a bunch of general
    /// soundness holes that Rust can't always sanely guard against.  These include:
    ///
    /// *   Unsound Direct3D API designs (e.g. shader creation functions that take a pointer and no length boundary!)
    /// *   Direct3D bugs (e.g. failing allocations may cause segfaults and worse)
    /// *   Kernel bugs (e.g. windows may BSOD)
    /// *   Driver bugs (e.g. gpus may hang/reset/???)
    ///
    /// The `unsafe` of this fn is the token acknowledgement of those errors.
    pub unsafe fn create(sdk_version: SdkVersion) -> Result<Self, MethodError> {
        let mut d3d9ex = null_mut();
        let hr = Direct3DCreate9Ex(sdk_version.into(), &mut d3d9ex);
        MethodError::check("Direct3DCreate9Ex", hr)?;
        Ok(Self(Rc::from_raw(d3d9ex)))
    }

    // TODO:
    // check_device_ex
    // enum_adapter_modes_ex
    // get_adapter_display_mode_ex
    // get_adapter_luid
    // get_adapter_mode_count_ex
}

#[test] fn create() {
    use winapi::shared::d3d9::D3D_SDK_VERSION;
    unsafe {
        let _ = Direct3DEx::create(SdkVersion::default().with_debug_disabled()).unwrap();
        let _ = Direct3DEx::create(SdkVersion::default().with_debug_enabled()).unwrap();
        let _ = Direct3DEx::create(SdkVersion::DEFAULT.with_debug_disabled()).unwrap();
        let _ = Direct3DEx::create(SdkVersion::DEFAULT.with_debug_enabled()).unwrap();
        let _ = Direct3DEx::create(SdkVersion::DEFAULT9B.with_debug_disabled()).unwrap();
        let _ = Direct3DEx::create(SdkVersion::DEFAULT9B.with_debug_enabled()).unwrap();
        let _ = Direct3DEx::create(SdkVersion::from(D3D_SDK_VERSION).with_debug_disabled()).unwrap();
        let _ = Direct3DEx::create(SdkVersion::from(D3D_SDK_VERSION).with_debug_enabled()).unwrap();
    }
}
