#![cfg(feature = "9ex")]

use crate::*;

use winapi::shared::d3d9::Direct3DCreate9Ex;
use winapi::shared::d3d9types::*;

use std::convert::TryInto;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9ex)\]
/// (extends [Direct3D])
/// Factory for use in creating your initial [DeviceEx].
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct Direct3DEx(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3D9Ex>);

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

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-createdeviceex)\]
    /// IDirect3D9Ex::CreateDeviceEx
    ///
    /// ### Safety
    ///
    /// *   The caller's codebase is responsible for ensuring any [HWND]s (`hwnd`, `presentation_parameters.hDeviceWindow`) outlive the [Device].
    ///      See [Direct3D::create_device] for guidance and details.
    /// *   `fullscreen_display_modes` is assumed to contain an entry for every adapter if `behavior_flags & D3DCREATE_ADAPTERGROUP_DEVICE` (TODO: enforce this via checks?)
    pub unsafe fn create_device_ex(&self, adapter: u32, device_type: impl Into<DevType>, hwnd: HWND, behavior_flags: impl Into<Create>, presentation_parameters: &mut D3DPRESENT_PARAMETERS, fullscreen_display_modes: &mut [D3DDISPLAYMODEEX]) -> Result<DeviceEx, MethodError> {
        for fdm in fullscreen_display_modes.iter_mut() {
            fdm.Size = std::mem::size_of_val(fdm).try_into().unwrap();
        }

        // TODO: examples, returns, etc.
        let mut device = null_mut();
        let modes = if fullscreen_display_modes.is_empty() { null_mut() } else { fullscreen_display_modes.as_mut_ptr() };
        let hr = self.0.CreateDeviceEx(adapter, device_type.into().into(), hwnd, behavior_flags.into().into(), presentation_parameters, modes, &mut device);
        MethodError::check("IDirect3D9Ex::CreateDeviceEx", hr)?;
        Ok(DeviceEx::from_raw(device))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-enumadaptermodesex)\]
    /// IDirect3D9Ex::EnumAdapterModesEx
    pub fn enum_adapter_modes_ex(&self, adapter: u32, filter: impl Into<D3DDISPLAYMODEFILTER>, mode: u32) -> Result<D3DDISPLAYMODEEX, MethodError> {
        let mut filter = filter.into();
        filter.Size = std::mem::size_of_val(&filter).try_into().unwrap();
        let mut dmex = unsafe { std::mem::zeroed::<D3DDISPLAYMODEEX>() };
        let hr = unsafe { self.0.EnumAdapterModesEx(adapter, &filter, mode, &mut dmex) };
        MethodError::check("IDirect3D9Ex::EnumAdapterModesEx", hr)?;
        Ok(dmex)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadapterdisplaymodeex)\]
    /// IDirect3D9Ex::GetAdapterDisplayModeEx
    pub fn get_adapter_display_mode_ex(&self, adapter: u32) -> Result<(D3DDISPLAYMODEEX, D3DDISPLAYROTATION), MethodError> {
        let mut mode = unsafe { std::mem::zeroed::<D3DDISPLAYMODEEX>() };
        mode.Size = std::mem::size_of_val(&mode).try_into().unwrap();
        let mut rot = D3DDISPLAYROTATION_IDENTITY;
        let hr = unsafe { self.0.GetAdapterDisplayModeEx(adapter, &mut mode, &mut rot) };
        MethodError::check("IDirect3D9Ex::GetAdapterDisplayModeEx", hr)?;
        Ok((mode, rot))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadapterluid)\]
    /// IDirect3D9Ex::GetAdapterLUID
    ///
    /// This method returns a unique identifier for the adapter that is specific to the adapter hardware.
    /// Applications can use this identifier to define robust mappings across various APIs (Direct3D 9, DXGI).
    pub fn get_adapter_luid(&self, adapter: u32) -> Result<Luid, MethodError> {
        let mut luid = Luid::default();
        let hr = unsafe { self.0.GetAdapterLUID(adapter, &mut *luid) };
        MethodError::check("IDirect3D9Ex::GetAdapterLUID", hr)?;
        Ok(luid)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadaptermodecountex)\]
    /// IDirect3D9Ex::GetAdapterModeCountEx
    ///
    /// Returns the number of display modes available.
    pub fn get_adapter_mode_count_ex(&self, adapter: u32, filter: impl Into<D3DDISPLAYMODEFILTER>) -> u32 {
        let mut filter = filter.into();
        filter.Size = std::mem::size_of_val(&filter).try_into().unwrap();
        unsafe { self.0.GetAdapterModeCountEx(adapter, &filter) }
    }
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

// TODO: so much more test coverage!
