use crate::*;
use crate::d3d9::*;

use bytemuck::Zeroable;

use winapi::shared::d3d9::{Direct3DCreate9, IDirect3D9};
use winapi::shared::windef::HMONITOR;
use winapi::um::unknwnbase::IUnknown;

use std::ptr::null_mut;



type AdapterIndex   = u32;
type ModeIndex      = u32;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9)\]
/// Factory for use in creating your initial [Device].
///
/// Microsoft's documentation claims that several APIs return [D3DERR::NOTAVAILABLE] when, in my testing, they return [D3DERR::INVALIDCALL] instead.
/// Do not trust the greyed out, crossed out, air quoted documentation <span style="opacity: 25%">like this</span>!
#[derive(Clone)] #[repr(transparent)]
pub struct Direct3D(pub(crate) mcom::Rc<IDirect3D9>);

unsafe impl AsSafe<IUnknown     > for Direct3D { fn as_safe(&self) -> &IUnknown     { &**self.0 } }
unsafe impl AsSafe<IDirect3D9   > for Direct3D { fn as_safe(&self) -> &IDirect3D9   { &*self.0 } }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9)\]
/// IDirect3D9 extension methods
///
/// ### Methods
/// | thindx                                                                    | learn.microsoft.com            | Description |
/// | ------------------------------------------------------------------------- | ----------------------------- | ----------- |
/// | [create](Self::create)                                                    | [Direct3DCreate9]             | Creates an [IDirect3D9] object and returns it.
/// | [check_depth_stencil_match](Self::check_depth_stencil_match)              | [CheckDepthStencilMatch]      | Determines whether a depth-stencil format is compatible with a render-target format in a particular display mode.
/// | [check_device_format](Self::check_device_format)                          | [CheckDeviceFormat]           | Determines whether a surface format is available as a specified resource type and can be used as a texture, depth-stencil buffer, or render target, or any combination of the three, on a device representing this adapter.
/// | [check_device_format_conversion](Self::check_device_format_conversion)    | [CheckDeviceFormatConversion] | Tests the device to see if it supports conversion from one display format to another.
/// | [check_device_multi_sample_type](Self::check_device_multi_sample_type)    | [CheckDeviceMultiSampleType]  | Determines if a multisampling technique is available on this device.
/// | [check_device_type](Self::check_device_type)                              | [CheckDeviceType]             | Verifies whether a hardware accelerated device type can be used on this adapter.
/// | [create_device](Self::create_device)                                      | [CreateDevice]                | Creates a [Device].
/// | [enum_adapter_modes](Self::enum_adapter_modes)                            | [EnumAdapterModes]            | Queries the possible display modes of an adapter (~ connected monitor)
/// | [get_adapter_count](Self::get_adapter_count)                              | [GetAdapterCount]             | Gets the number of adapters (~ connected monitors) available to this device.
/// | [get_adapter_display_mode](Self::get_adapter_display_mode)                | [GetAdapterDisplayMode]       | Gets the current display mode of an adapter (~ connected monitor)
/// | [get_adapter_identifier](Self::get_adapter_identifier)                    | [GetAdapterIdentifier]        | Gets metadata about an adapter (~ connected monitor), including driver name/version/guids/...
/// | [get_adapter_mode_count](Self::get_adapter_mode_count)                    | [GetAdapterModeCount]         | Get the number of display modes this adapter/monitor supports.
/// | [get_adapter_monitor](Self::get_adapter_monitor)                          | [GetAdapterMonitor]           | Get the [HMONITOR] associated with this adapter.
/// | [get_device_caps](Self::get_device_caps)                                  | [GetDeviceCaps]               | Get the [Caps] of this device.
/// | ~~register_software_device~~ (N/A)                                        | [RegisterSoftwareDevice]      | Registers a pluggable software device.
///
/// [Direct3DCreate9]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-direct3dcreate9
/// [IDirect3D9]:                   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9
///
/// [CheckDepthStencilMatch]:       https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdepthstencilmatch
/// [CheckDeviceFormat]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdeviceformat
/// [CheckDeviceFormatConversion]:  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdeviceformatconversion
/// [CheckDeviceMultiSampleType]:   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdevicemultisampletype
/// [CheckDeviceType]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdevicetype
/// [CreateDevice]:                 https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-createdevice
/// [EnumAdapterModes]:             https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-enumadaptermodes
/// [GetAdapterCount]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadaptercount
/// [GetAdapterDisplayMode]:        https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadapterdisplaymode
/// [GetAdapterIdentifier]:         https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadapteridentifier
/// [GetAdapterModeCount]:          https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadaptermodecount
/// [GetAdapterMonitor]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadaptermonitor
/// [GetDeviceCaps]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getdevicecaps
/// [RegisterSoftwareDevice]:       https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-registersoftwaredevice
///
pub trait IDirect3D9Ext : AsSafe<IDirect3D9> + Sized {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-direct3dcreate9)\]
    /// Direct3DCreate9
    ///
    /// Creates an [IDirect3D9] object and returns it.
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
    ///
    /// ### Returns
    /// *   [THINERR::NONSPECIFIC]  on an invalid SDK version
    /// *   [THINERR::NONSPECIFIC]  if d3d access is forbidden? (services?)
    /// *   Ok([Direct3D])          otherwise
    ///
    /// Consider calling [Direct3DEx::create_ex] instead if you want a meaningful error code than [THINERR::NONSPECIFIC].
    ///
    /// ### Example
    /// ```rust
    /// use thindx::d3d9::*;
    /// let d3d = unsafe { Direct3D::create(SdkVersion::default()) }.unwrap();
    /// ```
    ///
    /// [IDirect3D9]:                   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9
    ///
    unsafe fn create(sdk_version: SdkVersion) -> Result<Self, Error> where Self : From<mcom::Rc<IDirect3D9>> {
        fn_context!(d3d9::IDirect3D9Ext::create => Direct3DCreate9);
        let d3d9 = unsafe { Direct3DCreate9(sdk_version.into()) };
        unsafe { mcom::Rc::from_raw_opt(d3d9) }.ok_or(fn_error!(THINERR::NONSPECIFIC)).map(Self::from)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdepthstencilmatch)\]
    /// IDirect3D9::CheckDepthStencilMatch
    ///
    /// Determines whether a depth-stencil format is compatible with a render-target format in a particular display mode.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]   on most bad parameters
    /// *   <span style="opacity: 25%">[D3DERR::NOTAVAILABLE] "If a depth-stencil format is not compatible"</span>
    /// *   `Ok(())`                if the adapter x render target x depth stencil format combination is supported
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*;
    /// # let d3d = d3d_test();
    /// assert!([Format::D24S8, Format::D24X8, Format::D16, Format::D32].iter().copied().any(|fmt|
    ///     d3d.check_depth_stencil_match(
    ///         D3DADAPTER_DEFAULT,
    ///         DevType::HAL,
    ///         Format::X8R8G8B8,   // adapter format
    ///         Format::A8R8G8B8,   // render target format
    ///         fmt,                // depth stencil format
    ///     ).is_ok()
    /// ));
    /// ```
    fn check_depth_stencil_match(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, adapter_format: impl Into<Format>, render_target_format: impl Into<Format>, depth_stencil_format: impl Into<Format>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3D9Ext::check_depth_stencil_match => IDirect3D9::CheckDepthStencilMatch);
        fn_check_hr!(unsafe { self.as_winapi().CheckDepthStencilMatch(adapter, device_type.into().into(), adapter_format.into().into(), render_target_format.into().into(), depth_stencil_format.into().into()) })?;
        Ok(())
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdeviceformat)\]
    /// IDirect3D9::CheckDeviceFormat
    ///
    /// Determines whether a surface format is available as a specified resource type and can be used as a texture, depth-stencil buffer, or render target, or any combination of the three, on a device representing this adapter.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]       on most bad parameters
    /// *   <span style="opacity: 25%">[D3DERR::NOTAVAILABLE] "if the format is not acceptable to the device for this usage"</span>
    /// *   `Ok(())`                    if the format is supported
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// assert!(d3d.check_device_format(
    ///     D3DADAPTER_DEFAULT,
    ///     DevType::HAL,
    ///     Format::X8R8G8B8,       // adapter format
    ///     0,                      // usage
    ///     ResourceType::Texture,
    ///     Format::A8R8G8B8        // texture format
    /// ).is_ok());
    /// ```
    fn check_device_format(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, adapter_format: impl Into<Format>, usage: u32, rtype: impl Into<ResourceType>, check_format: impl Into<Format>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3D9Ext::check_device_format => IDirect3D9::CheckDeviceFormat);
        fn_check_hr!(unsafe { self.as_winapi().CheckDeviceFormat(adapter, device_type.into().into(), adapter_format.into().into(), usage, rtype.into().into(), check_format.into().into()) })?;
        Ok(())
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdeviceformatconversion)\]
    /// IDirect3D9::CheckDeviceFormatConversion
    ///
    /// Tests the device to see if it supports conversion from one display format to another.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDDEVICE]     if `device_type` is invalid
    /// *   [D3DERR::NOTAVAILABLE]      on an invalid source format
    /// *   [D3DERR::NOTAVAILABLE]      on an invalid target format
    /// *   [D3DERR::NOTAVAILABLE]      when the device cannot convert between the formats
    /// *   `Ok(())`                    if the device can convert from `source_format` to `target_format`
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// assert!(d3d.check_device_format_conversion(
    ///     D3DADAPTER_DEFAULT,
    ///     DevType::HAL,
    ///     Format::A8R8G8B8,
    ///     Format::A8R8G8B8
    /// ).is_ok());
    /// ```
    fn check_device_format_conversion(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, source_format: impl Into<Format>, target_format: impl Into<Format>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3D9Ext::check_device_format_conversion => IDirect3D9::CheckDeviceFormatConversion);
        fn_check_hr!(unsafe { self.as_winapi().CheckDeviceFormatConversion(adapter, device_type.into().into(), source_format.into().into(), target_format.into().into()) })?;
        Ok(())
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdevicemultisampletype)\]
    /// IDirect3D9::CheckDeviceMultiSampleType
    ///
    /// Determines if a multisampling technique is available on this device.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::NOTAVAILABLE]  on invalid surface formats, unless `D3DMULTISAMPLE_NONE`
    /// *   `Ok(quality_levels)`    if the multisampling type is valid for the given format
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// assert!(d3d.check_device_multi_sample_type(
    ///     D3DADAPTER_DEFAULT,
    ///     DevType::HAL,
    ///     Format::A8R8G8B8,
    ///     true,
    ///     MultiSample::None,
    /// ).is_ok());
    /// ```
    fn check_device_multi_sample_type(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, surface_format: impl Into<Format>, windowed: bool, multi_sample_type: impl Into<MultiSample>) -> Result<u32, Error> {
        fn_context!(d3d9::IDirect3D9Ext::check_device_multi_sample_type => IDirect3D9::CheckDeviceMultiSampleType);
        let mut quality_levels = 0;
        fn_check_hr!(unsafe { self.as_winapi().CheckDeviceMultiSampleType(adapter, device_type.into().into(), surface_format.into().into(), windowed.into(), multi_sample_type.into().into(), &mut quality_levels) })?;
        Ok(quality_levels)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdevicetype)\]
    /// IDirect3D9::CheckDeviceType
    ///
    /// Verifies whether a hardware accelerated device type can be used on this adapter.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]   if `device_type` is invalid
    /// *   [D3DERR::NOTAVAILABLE]  if `adapter_format` x `back_buffer_format` x `windowed` is invalid
    /// *   `Ok(())`                otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// assert!(d3d.check_device_type(
    ///     D3DADAPTER_DEFAULT, // adapter
    ///     DevType::HAL,       // device_type
    ///     Format::X8R8G8B8,   // adapter_format
    ///     Format::A8R8G8B8,   // back_buffer_format
    ///     true,               // windowed
    /// ).is_ok());
    /// ```
    fn check_device_type(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, adapter_format: impl Into<Format>, back_buffer_format: impl Into<Format>, windowed: bool) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3D9Ext::check_device_type => IDirect3D9::CheckDeviceType);
        fn_check_hr!(unsafe { self.as_winapi().CheckDeviceType(adapter, device_type.into().into(), adapter_format.into().into(), back_buffer_format.into().into(), windowed.into()) })?;
        Ok(())
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-createdevice)\]
    /// IDirect3D9::CreateDevice
    ///
    /// Creates a [Device].
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   The caller's codebase is responsible for ensuring any [HWND]s outlive the [Device].
    ///     This is effectively impossible to enforce at compile time.
    ///     Windows can typically destroy themselves without warning at any time.
    ///     Middleware can acquire [Device]s and hold onto them indefinitely with static data.
    ///     Graphics debuggers can acquire and leak [Device]s as well.
    ///
    ///     Instead, I recommend catching this at runtime.  You can check a [Device]'s refcount when freeing it from
    ///     within [WM_DESTROY], and explicitly exploding if it wasn't the last reference.  "Explicitly exploding" might include:
    ///     *   Printing diagnostics
    ///     *   [Triggering breakpoints](https://docs.rs/winapi/0.3/winapi/um/debugapi/fn.DebugBreak.html)
    ///     *   [Terminating the Process](https://docs.rs/winapi/0.3/winapi/um/errhandlingapi/fn.RaiseFailFastException.html)
    ///         (panics may be caught / cross WndProc ABI boundaries, and are insufficient for soundness!)
    ///
    ///     This will make device lifetime bugs obvious and shallow even in the face of non-rust code, *and* avoids
    ///     time sinks like self borrowing structs that come about if you attempt to abuse the borrow checker to
    ///     "enforce" safety.
    ///
    /// *   ???
    ///
    /// ### Errors
    /// *   [D3DERR::INVALIDCALL]       - If `behavior_flags` is missing [Create::FpuPreserve] (would be undefined behavior)
    /// *   [D3DERR::INVALIDCALL]       - Various other invalid parameters
    /// *   ???
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// let device = unsafe { d3d.create_device(
    ///     D3DADAPTER_DEFAULT,     // adapter
    ///     DevType::HAL,           // device_type
    ///     None,                   // focus_window
    ///     Create::FpuPreserve,    // behavior_flags
    ///     &mut PresentParameters {
    ///         // In/Out paramters - if these are 0 before the method create_device
    ///         // is called, they will be changed when create_device returns
    ///         back_buffer_width:              0,
    ///         back_buffer_height:             0,
    ///         back_buffer_count:              0,
    ///
    ///         // In/Out - if this equals Format::Unknown, it will be changed when
    ///         // create_device returns
    ///         back_buffer_format:             Format::X8R8G8B8,
    ///
    ///         // In parameters
    ///         multi_sample_type:              MultiSample::None,
    ///         multi_sample_quality:           0,
    ///         swap_effect:                    SwapEffect::Discard,
    ///         device_window:                  None,
    ///         windowed:                       true.into(),
    ///         enable_auto_depth_stencil:      false.into(),
    ///         auto_depth_stencil_format:      Format::Unknown,
    ///         flags:                          PresentFlag::none(),
    ///         full_screen_refresh_rate_in_hz: 0,
    ///         presentation_interval:          Present::zeroed(),
    ///     }
    /// )};
    /// let _device = device; // XXX
    /// ```
    ///
    /// [WM_DESTROY]:           https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-destroy
    unsafe fn create_device(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, focus_window: impl AsHWND, behavior_flags: impl Into<Create>, present_parameters: &mut PresentParameters<'static>) -> Result<Device, Error> {
        fn_context!(d3d9::IDirect3D9Ext::create_device => IDirect3D9::CreateDevice);
        let behavior_flags = u32::from(behavior_flags.into());
        if behavior_flags & u32::from(Create::FpuPreserve) == 0 { return Err(fn_param_error!(behavior_flags, D3DERR::INVALIDCALL)); }
        // TODO: better doc comments
        let mut device = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateDevice(adapter, device_type.into().into(), focus_window.as_hwnd(), behavior_flags, present_parameters.as_mut(), &mut device) })?;
        Ok(unsafe { Device::from_raw(device) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-enumadaptermodes)\] IDirect3D9::EnumAdapterModes
    ///
    /// Queries the possible display modes of an adapter (~ connected monitor)
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]       if `mode` >= `self.get_adapter_mode_count(adapter, format)`
    /// *   Ok([DisplayMode])           otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// let adapter = 0;
    /// let fmt = Format::X8R8G8B8;
    /// for mode in 0..d3d.get_adapter_mode_count(adapter, fmt) {
    ///     let mode = d3d.enum_adapter_modes(adapter, fmt, mode).unwrap();
    ///     let DisplayMode { width, height, refresh_rate, format } = mode;
    ///     println!("{}x{} @ {}hz {:?}", width, height, refresh_rate, format);
    /// }
    /// ```
    ///
    /// ### Output
    /// ```text
    /// 480x640 @ 59hz Format(D3DFMT_X8R8G8B8)
    /// 480x640 @ 60hz Format(D3DFMT_X8R8G8B8)
    /// 480x640 @ 67hz Format(D3DFMT_X8R8G8B8)
    /// 480x640 @ 72hz Format(D3DFMT_X8R8G8B8)
    /// 480x640 @ 75hz Format(D3DFMT_X8R8G8B8)
    /// ...
    /// 2160x3840 @ 30hz Format(D3DFMT_X8R8G8B8)
    /// 2160x3840 @ 60hz Format(D3DFMT_X8R8G8B8)
    /// ```
    fn enum_adapter_modes(&self, adapter: AdapterIndex, format: impl Into<Format>, mode: ModeIndex) -> Result<DisplayMode, Error> {
        fn_context!(d3d9::IDirect3D9Ext::enum_adapter_modes => IDirect3D9::EnumAdapterModes);
        let mut dm = DisplayMode::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().EnumAdapterModes(adapter, format.into().into(), mode, &mut *dm) })?;
        Ok(dm)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadaptercount)\] IDirect3D9::GetAdapterCount
    ///
    /// Gets the number of adapters (~ connected monitors) available to this device.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// println!("{} adapters", d3d.get_adapter_count());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// 4 adapters
    /// ```
    fn get_adapter_count(&self) -> AdapterIndex {
        fn_context!(d3d9::IDirect3D9Ext::get_adapter_count => IDirect3D9::GetAdapterCount);
        unsafe { self.as_winapi().GetAdapterCount() } // Safety:  Seems 100% safe per unit testing bellow
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadapterdisplaymode)\] IDirect3D9::GetAdapterDisplayMode
    ///
    /// Gets the current display mode of an adapter (~ connected monitor)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// for adapter in 0..d3d.get_adapter_count() {
    ///     let mode = d3d.get_adapter_display_mode(adapter).unwrap();
    ///     let DisplayMode { width, height, refresh_rate, format } = mode;
    ///     println!("{}x{} @ {}hz {:?}", width, height, refresh_rate, format);
    /// }
    /// ```
    ///
    /// ### Output
    /// ```text
    /// 2160x3840 @ 60hz Format(D3DFMT_X8R8G8B8)
    /// 2160x3840 @ 60hz Format(D3DFMT_X8R8G8B8)
    /// 2160x3840 @ 29hz Format(D3DFMT_X8R8G8B8)
    /// 2160x3840 @ 29hz Format(D3DFMT_X8R8G8B8)
    /// ```
    fn get_adapter_display_mode(&self, adapter: AdapterIndex) -> Result<DisplayMode, Error> {
        fn_context!(d3d9::IDirect3D9Ext::get_adapter_display_mode => IDirect3D9::GetAdapterDisplayMode);
        let mut dm = DisplayMode::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetAdapterDisplayMode(adapter, &mut *dm) })?;
        Ok(dm)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadapteridentifier)\] IDirect3D9::GetAdapterIdentifier
    ///
    /// Gets metadata about an adapter (~ connected monitor), including driver name/version/guids/whql info/vendor/device ids/blood type/birthplace/???
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]       if `flags` isn't valid (0, D3DENUM_WHQL_LEVEL, ...)
    /// *   Ok([AdapterIdentifier])     otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// eprintln!("{:#?}", d3d.get_adapter_identifier(0, 0).unwrap());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// AdapterIdentifier {
    ///     Driver: "aticfx64.dll",
    ///     Description: "Radeon RX Vega M GL Graphics",
    ///     DeviceName: "\\\\.\\DISPLAY1",
    ///     DriverVersion: 0x1800142B1207D1,
    ///     VendorId: 0x1002,
    ///     DeviceId: 0x694E,
    ///     SubSysId: 0x20738086,
    ///     Revision: 0x000000C0,
    ///     DeviceIdentifier: "{...}",
    ///     WHQLLevel: 0,
    /// }
    /// ```
    fn get_adapter_identifier(&self, adapter: AdapterIndex, flags: u32) -> Result<AdapterIdentifier, Error> {
        fn_context!(d3d9::IDirect3D9Ext::get_adapter_identifier => IDirect3D9::GetAdapterIdentifier);
        let mut ident = AdapterIdentifier::default();
        fn_check_hr!(unsafe { self.as_winapi().GetAdapterIdentifier(adapter, flags, &mut *ident) })?;
        Ok(ident)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadaptermodecount)\] IDirect3D9::GetAdapterModeCount
    ///
    /// Get the number of display modes this adapter/monitor supports.
    ///
    /// ### Arguments
    /// *   `adapter`   - the adapter/monitor to query modes for
    /// *   `format`    - the adapter format to query modes for.  `Format::X8R8G8B8` likely works, many other formats likely don't.
    ///
    /// ### Returns
    /// *   `0`         if `adapter` >= `self.get_adapter_count()`
    /// *   `0`         if `format` is an invalid format
    /// *   `0`         if `format` is an unsupported format
    /// *   `n`         modes you can query [enum_adapter_modes] for
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// for fmt in [Format::X8R8G8B8, Format::A8R8G8B8].iter().copied() {
    ///     let modes = d3d.get_adapter_mode_count(0, fmt);
    ///     println!("adapter 1: {:?}: {} mode(s)", fmt, modes);
    /// }
    /// ```
    ///
    /// ### Output
    /// ```text
    /// adapter 1: Format(D3DFMT_X8R8G8B8): 44 mode(s)
    /// adapter 1: Format(D3DFMT_A8R8G8B8): 0 mode(s)
    /// ```
    ///
    /// ### See Also
    /// *   [enum_adapter_modes]
    ///
    /// [enum_adapter_modes]:       #method.enum_adapter_modes
    fn get_adapter_mode_count(&self, adapter: AdapterIndex, format: impl Into<Format>) -> ModeIndex {
        fn_context!(d3d9::IDirect3D9Ext::get_adapter_mode_count => IDirect3D9::GetAdapterModeCount);
        unsafe { self.as_winapi().GetAdapterModeCount(adapter, format.into().into()) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadaptermonitor)\] IDirect3D9::GetAdapterMonitor
    ///
    /// ### Returns
    /// *   [THINERR::NONSPECIFIC]  on an invalid `adapter`
    /// *   [THINERR::NONSPECIFIC]  if no monitor is connected to `adapter`?
    /// *   Ok([HMONITOR])          otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// let monitor : HMONITOR  = d3d.get_adapter_monitor(   0).unwrap();
    /// let error               = d3d.get_adapter_monitor(9001).unwrap_err();
    /// ```
    fn get_adapter_monitor(&self, adapter: AdapterIndex) -> Result<HMONITOR, Error> {
        fn_context!(d3d9::IDirect3D9Ext::get_adapter_monitor => IDirect3D9::GetAdapterMonitor);
        let hm = unsafe { self.as_winapi().GetAdapterMonitor(adapter) }; // Safety: Seems to be safe even when `adapter` >= `self.get_adapter_count()` per unit test bellow
        if hm.is_null() { Err(fn_error!(THINERR::NONSPECIFIC)) } else { Ok(hm) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getdevicecaps)\] IDirect3D9::GetDeviceCaps
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       on an invalid `adapter`
    /// *   [D3DERR::INVALIDDEVICE]     on an invalid `device_type`
    /// *   Ok([Caps])                  otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let d3d = d3d_test();
    /// let caps : Caps = d3d.get_device_caps(0, DevType::HAL).unwrap();
    /// assert_eq!(caps.device_type,        DevType::HAL);
    /// assert_eq!(caps.adapter_ordinal,    0);
    /// assert!(caps.max_texture_width    > 0);
    /// assert!(caps.max_texture_height   > 0);
    /// // ...
    /// ```
    fn get_device_caps(&self, adapter: AdapterIndex, device_type: DevType) -> Result<Caps, Error> {
        fn_context!(d3d9::IDirect3D9Ext::get_device_caps => IDirect3D9::GetDeviceCaps);
        let mut caps = Caps::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetDeviceCaps(adapter, device_type.into(), &mut *caps) })?; // Safety: Appears sound on all invalid parameters per unit testing
        Ok(caps)
    }

    // TODO: IDirect3D9::RegisterSoftwareDevice = d3d9::IDirect3D9Ext::register_software_device
    // I can't easily find proper documentation demonstrating usage.  Supposedly it's buried in a DDK somewhere.
    // Pull requests welcome w/ unit testing coverage!
}

impl<T: AsSafe<IDirect3D9> + Sized> IDirect3D9Ext for T {}



#[cfg(test)] mod tests {
    use dev::d3d9::*;
    use std::ptr::null_mut;

    #[test] fn create() {
        use winapi::shared::d3d9::D3D_SDK_VERSION;
        unsafe {
            Direct3D::create(SdkVersion::default()).unwrap();
            Direct3D::create(SdkVersion::default().with_debug_disabled()).unwrap();
            Direct3D::create(SdkVersion::default().with_debug_enabled()).unwrap();
            Direct3D::create(SdkVersion::DEFAULT.with_debug_disabled()).unwrap();
            Direct3D::create(SdkVersion::DEFAULT.with_debug_enabled()).unwrap();
            Direct3D::create(SdkVersion::DEFAULT9B.with_debug_disabled()).unwrap();
            Direct3D::create(SdkVersion::DEFAULT9B.with_debug_enabled()).unwrap();
            Direct3D::create(SdkVersion::from(D3D_SDK_VERSION).with_debug_disabled()).unwrap();
            Direct3D::create(SdkVersion::from(D3D_SDK_VERSION).with_debug_enabled()).unwrap();
            assert_eq!(THINERR::NONSPECIFIC, Direct3D::create(SdkVersion::from(0)                       ).map(|_|()));
            assert_eq!(THINERR::NONSPECIFIC, Direct3D::create(SdkVersion::from(0).with_debug_disabled() ).map(|_|()));
            assert_eq!(THINERR::NONSPECIFIC, Direct3D::create(SdkVersion::from(0).with_debug_enabled()  ).map(|_|()));
        }
    }

    #[test] fn check_depth_stencil_match() {
        let d3d = d3d_test();
        assert!(                        d3d.check_depth_stencil_match(0,    DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, Format::D24S8  ).is_ok()); // valid
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(9001, DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, Format::D24S8  )); // invalid adapter
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(0,    Invalid,      Format::X8R8G8B8, Format::A8R8G8B8, Format::D24S8  )); // invalid devtype
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(0,    DevType::HAL, Format::UNKNOWN,  Format::A8R8G8B8, Format::D24S8  )); // invalid adapter format
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(0,    DevType::HAL, Format::X8R8G8B8, Format::UNKNOWN,  Format::D24S8  )); // invalid render target format
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(0,    DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, Format::UNKNOWN)); // invalid depth stencil format
    }

    #[test] fn check_device_format() {
        let d3d = d3d_test();
        assert!(                        d3d.check_device_format(0,    DevType::HAL, Format::X8R8G8B8,  0, ResourceType::Texture, Format::A8R8G8B8).is_ok()); // valid
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(9001, DevType::HAL, Format::X8R8G8B8,  0, ResourceType::Texture, Format::A8R8G8B8)); // invalid adapter
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    Invalid,      Format::X8R8G8B8,  0, ResourceType::Texture, Format::A8R8G8B8)); // invalid devtype
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    DevType::HAL, Format::UNKNOWN,   0, ResourceType::Texture, Format::A8R8G8B8)); // invalid adapter format
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    DevType::HAL, Format::X8R8G8B8, !0, ResourceType::Texture, Format::A8R8G8B8)); // invalid usage
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    DevType::HAL, Format::X8R8G8B8,  0, Invalid,               Format::A8R8G8B8)); // invalid resource type
        assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    DevType::HAL, Format::X8R8G8B8,  0, ResourceType::Texture, Format::UNKNOWN )); // invalid check format
    }

    #[test] fn check_device_format_conversion() {
        let d3d = d3d_test();
        assert!(                            d3d.check_device_format_conversion(0,    DevType::HAL, Format::A8R8G8B8, Format::A8R8G8B8).is_ok()); // valid
        assert!(                            d3d.check_device_format_conversion(0,    DevType::HAL, Format::D24S8,    Format::D24S8   ).is_ok()); // valid
        assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_format_conversion(9001, DevType::HAL, Format::A8R8G8B8, Format::A8R8G8B8)); // invalid adapter
        assert_eq!(D3DERR::INVALIDDEVICE,   d3d.check_device_format_conversion(0,    Invalid,      Format::A8R8G8B8, Format::A8R8G8B8)); // invalid devtype
        assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_format_conversion(0,    DevType::HAL, Invalid,          Format::A8R8G8B8)); // invalid source format
        assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_format_conversion(0,    DevType::HAL, Format::A8R8G8B8, Invalid         )); // invalid target format
        assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_format_conversion(0,    DevType::HAL, Format::D24S8,    Format::A8R8G8B8)); // invalid conversion
        assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_format_conversion(0,    DevType::HAL, Format::A8R8G8B8, Format::D24S8   )); // invalid conversion
    }

    #[test] fn check_device_multi_sample_type() {
        let d3d = d3d_test();
        assert!(                            d3d.check_device_multi_sample_type(0,    DevType::HAL, Format::A8R8G8B8, true, MultiSample::None    ).is_ok()); // valid
        assert!(                            d3d.check_device_multi_sample_type(0,    DevType::HAL, Format::A8R8G8B8, true, MultiSample::X2      ).is_ok()); // valid
        assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_multi_sample_type(9001, DevType::HAL, Format::A8R8G8B8, true, MultiSample::X2      )); // invalid adapter
        assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_multi_sample_type(0,    Invalid,      Format::A8R8G8B8, true, MultiSample::X2      )); // invalid devtype
        assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_multi_sample_type(0,    DevType::HAL, Invalid,          true, MultiSample::X2      )); // invalid surface format
        let _ =                             d3d.check_device_multi_sample_type(0,    DevType::HAL, Invalid,          true, MultiSample::None     ); // can succeed despite invalid format when we use D3DMULTISAMPLE_NONE
        assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_multi_sample_type(0,    DevType::HAL, Format::A8R8G8B8, true, Invalid              )); // invalid multisampling
    }

    #[test] fn check_device_type() {
        let d3d = d3d_test();
        assert!(                            d3d.check_device_type(0,    DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, true).is_ok()); // valid
        assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_type(9001, DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, true)); // invalid adapter
        assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_type(0,    Invalid,      Format::X8R8G8B8, Format::A8R8G8B8, true)); // invalid devtype
        assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_type(0,    DevType::HAL, Invalid,          Format::A8R8G8B8, true)); // invalid adapter format
        assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_type(0,    DevType::HAL, Format::X8R8G8B8, Invalid,          true)); // invalid back buffer format
    }

    // #[test] fn create_device() {} // TODO

    #[test] fn enum_adapter_modes() {
        let d3d = d3d_test();
        let adapters = d3d.get_adapter_count();

        // all adapters should be valid
        for adapter in 0..adapters {
            eprintln!("checking adapter {} of {}", adapter+1, adapters);
            for fmt in [
                Format::UNKNOWN, Format::R8G8B8, Format::A8R8G8B8, Format::X8R8G8B8, Format::A8B8G8R8, Format::X8B8G8R8,
                Format::from_unchecked(1), Format::from_unchecked(10000), Format::from_unchecked(!0),
            ].iter().copied() {
                let modes = d3d.get_adapter_mode_count(adapter, fmt);
                for mode in 0..modes {
                    d3d.enum_adapter_modes(adapter, fmt, mode).unwrap_or_else(|err| panic!("enum_adapter_modes({}, {:?}, {}) failed: {}", adapter, fmt, mode, err));
                }
                for mode in modes..modes+100 {
                    assert_eq!(D3DERR::INVALIDCALL, d3d.enum_adapter_modes(adapter, fmt, mode));
                }
            }
        }

        // doublecheck that d3d doesn't segfault for out-of-bounds adapters or anything
        for adapter in [adapters+0, adapters+100, adapters+100000, adapters+10000000].iter().copied() {
            eprintln!("checking invalid adapter {} of {}", adapter+1, adapters);
            assert_eq!(D3DERR::INVALIDCALL, d3d.enum_adapter_modes(adapter, Format::X8R8G8B8, 0));
            assert_eq!(D3DERR::INVALIDCALL, d3d.enum_adapter_modes(adapter, Format::X8R8G8B8, 100));
            assert_eq!(D3DERR::INVALIDCALL, d3d.enum_adapter_modes(adapter, Format::X8R8G8B8, 10000));
        }
    }

    #[test] fn get_adapter_count() {
        let d3d = d3d_test();
        assert!(d3d.get_adapter_count() > 0);
    }

    #[test] fn get_adapter_display_mode() {
        let d3d = d3d_test();
        let adapters = d3d.get_adapter_count();
        for adapter in 0..adapters {
            d3d.get_adapter_display_mode(adapter).unwrap_or_else(|err| panic!("unable to query display mode of adapter {} of {}: {}", adapter+1, adapters, err));
        }
        for adapter in adapters..(100+adapters) {
            assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_display_mode(adapter));
        }
    }

    #[test] fn get_adapter_identifier() {
        let d3d = d3d_test();

        /// The D3DENUM_WHQL_LEVEL value has been retired for 9Ex and future versions
        ///
        /// Ref: `d3d9.h`
        const D3DENUM_WHQL_LEVEL : u32 = 2;

        let valid = 0;
        d3d.get_adapter_identifier(valid, 0                 ).unwrap();
        d3d.get_adapter_identifier(valid, D3DENUM_WHQL_LEVEL).unwrap();
        assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(valid, D3DENUM_WHQL_LEVEL+1));
        assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(valid, D3DENUM_WHQL_LEVEL+1000));
        assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(valid, D3DENUM_WHQL_LEVEL+1000000));

        let invalid = 9001;
        assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(invalid, 0                   ));
        assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(invalid, D3DENUM_WHQL_LEVEL  ));
    }

    #[test] fn get_adapter_mode_count() {
        let d3d = d3d_test();
        let adapters = d3d.get_adapter_count();
        for valid in 0..adapters {
            for (fmt,                           expect) in [
                (Format::UNKNOWN,               Some(false)),
                (Format::from_unchecked(1),     Some(false)),
                (Format::R8G8B8,                None),
                (Format::A8R8G8B8,              None),
                (Format::X8R8G8B8,              Some(true)),
                (Format::A8B8G8R8,              None),
                (Format::X8B8G8R8,              None),
                (Format::from_unchecked(10000), Some(false)),
                (Format::from_unchecked(!0),    Some(false)),
            ].iter().copied() {
                let modes = d3d.get_adapter_mode_count(valid, fmt);
                assert!(expect.unwrap_or(true)   || modes == 0, "adapter {} of {}: {:?} has {} modes, none expected", valid+1, adapters, fmt, modes);
                assert!(!expect.unwrap_or(false) || modes >  0, "adapter {} of {}: {:?} has {} modes, some expected", valid+1, adapters, fmt, modes);
            }
        }
        for invalid in [adapters+0, adapters+100, adapters+100000, adapters+10000000].iter().copied() {
            for fmt in [Format::UNKNOWN, Format::X8R8G8B8].iter() {
                assert_eq!(0, d3d.get_adapter_mode_count(invalid, Format::UNKNOWN ), "adapter {} of {} has modes for format {:?} despite being out-of-bounds", invalid+1, adapters, fmt);
            }
        }
    }

    #[test] fn get_adapter_monitor() {
        let d3d = d3d_test();
        let adapters = d3d.get_adapter_count();
        for valid in 0..adapters                { assert!(!d3d.get_adapter_monitor(  valid).unwrap_or(null_mut()).is_null(), "adapter {} of {} has a null HMONITOR", valid+1, adapters); }
        for invalid in adapters..adapters+100   { assert!( d3d.get_adapter_monitor(invalid).is_err(),                        "adapter {} of {} returned an HMONITOR despite being out-of-bounds!", invalid+1, adapters); }
    }

    #[test] fn get_device_caps() {
        let d3d = d3d_test();

        let adapter = 0; // valid
        d3d.get_device_caps(adapter, DevType::HAL).unwrap();
        d3d.get_device_caps(adapter, DevType::Ref).unwrap();
        d3d.get_device_caps(adapter, DevType::NullRef).unwrap();
        assert_eq!(D3DERR::INVALIDDEVICE,   d3d.get_device_caps(adapter, DevType::from_unchecked(100)));
        assert_eq!(D3DERR::INVALIDDEVICE,   d3d.get_device_caps(adapter, DevType::from_unchecked(10000)));
        assert_eq!(D3DERR::INVALIDDEVICE,   d3d.get_device_caps(adapter, DevType::from_unchecked(1000000)));
        assert_eq!(D3DERR::INVALIDDEVICE,   d3d.get_device_caps(adapter, DevType::from_unchecked(100000000)));

        let adapter = 9001; // invalid
        assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::HAL));
        assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::Ref));
        assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::NullRef));
        assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::from_unchecked(100)));
        assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::from_unchecked(10000)));
        assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::from_unchecked(1000000)));
        assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::from_unchecked(100000000)));
    }
}




//#cpp2rust IDirect3D9                                          = d3d9::Direct3D
//#cpp2rust IDirect3D9                                          = d3d9::IDirect3D9Ext
