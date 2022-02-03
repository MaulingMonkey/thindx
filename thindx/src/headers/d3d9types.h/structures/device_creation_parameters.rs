use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;
use winapi::shared::windef::HWND;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddevice-creation-parameters)\]
/// D3DDEVICE_CREATION_PARAMETERS
///
/// Describes the creation parameters for a device.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::get_creation_parameters]
#[derive(Clone, Copy, Debug)]
#[derive(Zeroable)] // HWND is not Pod
#[repr(C)] pub struct DeviceCreationParameters {
    /// The ordinal number denoting the display adapter.
    /// `D3DADAPTER_DEFAULT` (0) is always the primary display adapter.
    /// This ordinal can be used with various [IDirect3D9Ext] methods.
    ///
    /// **NOTE:**
    /// *   Different [IDirect3D9Ext] instances can use different ordinals for the same adapters!
    /// *   Adapters enter and leave a system when users hot swap monitors, projectors, laptops, etc.!
    /// *   Avoid mixing/matching ordinals between D3D instances.
    /// *   Avoid storing ordinals long term?  (Store e.g. devices and re-query those instead?)
    pub adapter_ordinal:    u32,

    /// The [d3d::DevType] that was requested when calling e.g. [create_device](IDirect3D9Ext::create_device).
    pub device_type:        DevType,

    /// The window that was passed to [create_device](IDirect3D9Ext::create_device).
    pub focus_window:       HWND,

    /// The [d3d::Create] flags that were used when calling e.g. [create_device](IDirect3D9Ext::create_device).
    pub behavior_flags:     Create,
}

test_layout! { DeviceCreationParameters => D3DDEVICE_CREATION_PARAMETERS { adapter_ordinal => AdapterOrdinal, device_type => DeviceType, focus_window => hFocusWindow, behavior_flags => BehaviorFlags } }

impl Deref    for DeviceCreationParameters { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DDEVICE_CREATION_PARAMETERS; }
impl DerefMut for DeviceCreationParameters { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DDEVICE_CREATION_PARAMETERS> for DeviceCreationParameters { fn from(value: D3DDEVICE_CREATION_PARAMETERS) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<DeviceCreationParameters> for D3DDEVICE_CREATION_PARAMETERS { fn from(value: DeviceCreationParameters   ) -> Self { unsafe { std::mem::transmute(value) } } }

//#cpp2rust D3DDEVICE_CREATION_PARAMETERS = d3d::DeviceCreationParameters
