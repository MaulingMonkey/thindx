#![cfg_attr(not(feature = "9ex"), allow(unused_imports))]

use crate::*;
use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9::{Direct3DCreate9Ex, IDirect3D9Ex, IDirect3D9};
use winapi::shared::d3d9types::*;
use winapi::shared::windef::HWND;
use winapi::um::unknwnbase::IUnknown;

use std::convert::TryInto;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9ex)\]
/// (extends [Direct3D])
/// Factory for use in creating your initial [DeviceEx].
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct Direct3DEx(pub(crate) mcom::Rc<IDirect3D9Ex>);

#[cfg(feature = "9ex")] unsafe impl AsSafe<IUnknown     > for Direct3DEx { fn as_safe(&self) -> &IUnknown       { &***self.0 } }
#[cfg(feature = "9ex")] unsafe impl AsSafe<IDirect3D9   > for Direct3DEx { fn as_safe(&self) -> &IDirect3D9     { &**self.0 } }
#[cfg(feature = "9ex")] unsafe impl AsSafe<IDirect3D9Ex > for Direct3DEx { fn as_safe(&self) -> &IDirect3D9Ex   { &*self.0 } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9ex)\]
/// IDirect3D9Ex extension methods
///
/// ### Methods
/// | thindx                                                            | docs.microsoft.com        | Description |
/// | ----------------------------------------------------------------- | ------------------------- | ----------- |
/// | [create](Self::create)                                            | [Direct3DCreate9Ex]       | Creates an [IDirect3D9Ex] object and returns an interface to it.
/// | [create_device_ex](Self::create_device_ex)                        | [CreateDeviceEx]          | Creates a device to represent the display adapter.
/// | [enum_adapter_modes_ex](Self::enum_adapter_modes_ex)              | [EnumAdapterModesEx]      | Enumerate actual display mode info based on the given mode index.
/// | [get_adapter_display_mode_ex](Self::get_adapter_display_mode_ex)  | [GetAdapterDisplayModeEx] | Retrieves the current display mode and rotation settings of the adapter.
/// | [get_adapter_luid](Self::get_adapter_luid)                        | [GetAdapterLUID]          | This method returns a [Luid] for the adapter that is specific to the adapter hardware.
/// | [get_adapter_mode_count_ex](Self::get_adapter_mode_count_ex)      | [GetAdapterModeCountEx]   | Returns the number of display modes available.
///
/// [Direct3DCreate9Ex]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-direct3dcreate9ex
/// [IDirect3D9Ex]:         https://docs.microsoft.com/en-us/windows/desktop/api/d3d9/nn-d3d9-idirect3d9ex
///
/// [CreateDeviceEx]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-createdeviceex
/// [EnumAdapterModesEx]:       https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-enumadaptermodesex
/// [GetAdapterDisplayModeEx]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadapterdisplaymodeex
/// [GetAdapterLUID]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadapterluid
/// [GetAdapterModeCountEx]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadaptermodecountex
///
#[cfg(feature = "9ex")]
pub trait IDirect3D9ExExt : AsSafe<IDirect3D9Ex> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-direct3dcreate9ex)\]
    /// Direct3DCreate9Ex
    ///
    /// Creates an [IDirect3D9Ex](IDirect3D9ExExt) object and returns an interface to it.
    ///
    /// ### ⚠️ Safety ⚠️
    /// While this individual call should be 100% safe/sound, there is a bunch of general
    /// soundness holes that Rust can't always sanely guard against.  These include:
    ///
    /// *   Unsound Direct3D API designs (e.g. shader creation functions that take a pointer and no length boundary!)
    /// *   Direct3D bugs (e.g. failing allocations may cause segfaults and worse)
    /// *   Kernel bugs (e.g. windows may BSOD)
    /// *   Driver bugs (e.g. gpus may hang/reset/???)
    ///
    /// The `unsafe` of this fn is the token acknowledgement of those errors.
    unsafe fn create_ex(sdk_version: SdkVersion) -> Result<Self, Error> where Self : From<mcom::Rc<IDirect3D9Ex>> {
        fn_context!(d3d9::IDirect3D9ExExt::create_ex => Direct3DCreate9Ex);
        let mut d3d9ex = null_mut();
        fn_check_hr!(unsafe { Direct3DCreate9Ex(sdk_version.into(), &mut d3d9ex) })?;
        Ok(Self::from(unsafe { mcom::Rc::from_raw(d3d9ex) }))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-createdeviceex)\]
    /// IDirect3D9Ex::CreateDeviceEx
    ///
    /// Creates a device to represent the display adapter.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   The caller's codebase is responsible for ensuring any [HWND]s (`hwnd`, `presentation_parameters.hDeviceWindow`) outlive the [Device].
    ///     See [IDirect3D9Ext::create_device] for guidance and details.
    /// *   `fullscreen_display_modes` is assumed to contain an entry for every adapter if `behavior_flags & D3DCREATE_ADAPTERGROUP_DEVICE` (TODO: enforce this via checks?)
    ///
    /// ### Errors
    /// *   [D3DERR::INVALIDCALL]       - If `behavior_flags` is missing [Create::FpuPreserve] (would be undefined behavior)
    /// *   [D3DERR::INVALIDCALL]       - Various other invalid parameters
    /// *   ???
    unsafe fn create_device_ex(&self, adapter: u32, device_type: impl Into<DevType>, hwnd: HWND, behavior_flags: impl Into<Create>, presentation_parameters: &mut PresentParameters<'static>, fullscreen_display_modes: &mut [DisplayModeEx]) -> Result<DeviceEx, Error> {
        fn_context!(d3d9::IDirect3D9ExExt::create_device_ex => IDirect3D9Ex::CreateDeviceEx);
        let behavior_flags = u32::from(behavior_flags.into());
        if behavior_flags & u32::from(Create::FpuPreserve) == 0 { return Err(fn_param_error!(behavior_flags, D3DERR::INVALIDCALL)); }

        for fdm in fullscreen_display_modes.iter_mut() {
            fdm.size = std::mem::size_of_val(fdm).try_into().unwrap();
        }

        // TODO: examples, returns, etc.
        let mut device = null_mut();
        let modes = if fullscreen_display_modes.is_empty() { null_mut() } else { fullscreen_display_modes.as_mut_ptr().cast() };
        fn_check_hr!(unsafe { self.as_winapi().CreateDeviceEx(adapter, device_type.into().into(), hwnd, behavior_flags, presentation_parameters.as_mut(), modes, &mut device) })?;
        Ok(unsafe { DeviceEx::from_raw(device) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-enumadaptermodesex)\]
    /// IDirect3D9Ex::EnumAdapterModesEx
    ///
    /// Enumerate actual display mode info based on the given mode index.
    fn enum_adapter_modes_ex(&self, adapter: u32, filter: impl Into<DisplayModeFilter>, mode: u32) -> Result<DisplayModeEx, Error> {
        fn_context!(d3d9::IDirect3D9ExExt::enum_adapter_modes_ex => IDirect3D9Ex::EnumAdapterModesEx);
        let mut filter = filter.into();
        filter.size = std::mem::size_of_val(&filter).try_into().unwrap();
        let mut dmex = DisplayModeEx::zeroed();
        dmex.size = std::mem::size_of_val(&dmex).try_into().unwrap();
        fn_check_hr!(unsafe { self.as_winapi().EnumAdapterModesEx(adapter, filter.as_ref(), mode, dmex.as_mut()) })?;
        Ok(dmex)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadapterdisplaymodeex)\]
    /// IDirect3D9Ex::GetAdapterDisplayModeEx
    ///
    /// Retrieves the current display mode and rotation settings of the adapter.
    fn get_adapter_display_mode_ex(&self, adapter: u32) -> Result<(DisplayModeEx, DisplayRotation), Error> {
        fn_context!(d3d9::IDirect3D9ExExt::get_adapter_display_mode_ex => IDirect3D9Ex::GetAdapterDisplayModeEx);
        let mut mode = DisplayModeEx::zeroed();
        mode.size = std::mem::size_of_val(&mode).try_into().unwrap();
        let mut rot = D3DDISPLAYROTATION_IDENTITY;
        fn_check_hr!(unsafe { self.as_winapi().GetAdapterDisplayModeEx(adapter, mode.as_mut(), &mut rot) })?;
        Ok((mode, DisplayRotation::from_unchecked(rot)))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadapterluid)\]
    /// IDirect3D9Ex::GetAdapterLUID
    ///
    /// This method returns a unique identifier for the adapter that is specific to the adapter hardware.
    /// Applications can use this identifier to define robust mappings across various APIs (Direct3D 9, DXGI).
    fn get_adapter_luid(&self, adapter: u32) -> Result<Luid, Error> {
        fn_context!(d3d9::IDirect3D9ExExt::get_adapter_luid => IDirect3D9Ex::GetAdapterLUID);
        let mut luid = Luid::default();
        fn_check_hr!(unsafe { self.as_winapi().GetAdapterLUID(adapter, &mut *luid) })?;
        Ok(luid)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadaptermodecountex)\]
    /// IDirect3D9Ex::GetAdapterModeCountEx
    ///
    /// Returns the number of display modes available.
    fn get_adapter_mode_count_ex(&self, adapter: u32, filter: impl Into<DisplayModeFilter>) -> u32 {
        fn_context!(d3d9::IDirect3D9ExExt::get_adapter_mode_count_ex => IDirect3D9Ex::GetAdapterModeCountEx);
        let mut filter = filter.into();
        filter.size = std::mem::size_of_val(&filter).try_into().unwrap();
        unsafe { self.as_winapi().GetAdapterModeCountEx(adapter, filter.as_ref()) }
    }
}

#[cfg(feature = "9ex")] impl<T: AsSafe<IDirect3D9Ex>> IDirect3D9ExExt for T {}



#[cfg(feature = "9ex")] #[test] fn create() {
    use winapi::shared::d3d9::D3D_SDK_VERSION;
    unsafe {
        let _ = Direct3DEx::create_ex(SdkVersion::default().with_debug_disabled()).unwrap();
        let _ = Direct3DEx::create_ex(SdkVersion::default().with_debug_enabled()).unwrap();
        let _ = Direct3DEx::create_ex(SdkVersion::DEFAULT.with_debug_disabled()).unwrap();
        let _ = Direct3DEx::create_ex(SdkVersion::DEFAULT.with_debug_enabled()).unwrap();
        let _ = Direct3DEx::create_ex(SdkVersion::DEFAULT9B.with_debug_disabled()).unwrap();
        let _ = Direct3DEx::create_ex(SdkVersion::DEFAULT9B.with_debug_enabled()).unwrap();
        let _ = Direct3DEx::create_ex(SdkVersion::from(D3D_SDK_VERSION).with_debug_disabled()).unwrap();
        let _ = Direct3DEx::create_ex(SdkVersion::from(D3D_SDK_VERSION).with_debug_enabled()).unwrap();
    }
}

// TODO: so much more test coverage!



//#cpp2rust IDirect3D9Ex                            = d3d9::Direct3DEx
//#cpp2rust IDirect3D9Ex                            = d3d9::IDirect3D9ExExt
