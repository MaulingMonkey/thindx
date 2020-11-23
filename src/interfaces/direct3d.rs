use crate::*;

use winapi::shared::d3d9::Direct3DCreate9;
use winapi::shared::windef::HMONITOR;

use std::ptr::null_mut;



type AdapterIndex   = u32;
type ModeIndex      = u32;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9)\]
/// Factory for use in creating your initial [Device].
///
/// Microsoft's documentation claims that several APIs return [D3DERR::NOTAVAILABLE] when, in my testing, they return [D3DERR::INVALIDCALL] instead.
/// Do not trust the greyed out, crossed out, air quoted documentation <span class="inaccurate">like this</span>!
#[derive(Clone)] #[repr(transparent)]
pub struct Direct3D(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3D9>);



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-direct3dcreate9)\]
    /// Direct3DCreate9
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
    ///
    /// ### Returns
    ///
    /// *   `Err(())`       on an invalid SDK version
    /// *   `Err(())`       if d3d access is forbidden? (services?)
    /// *   Ok([Direct3D])  otherwise
    ///
    /// ### Example
    ///
    /// ```rust
    /// use thin3d9::*;
    /// let d3d = unsafe { Direct3D::create(SdkVersion::default()) }.unwrap();
    /// ```
    pub unsafe fn create(sdk_version: SdkVersion) -> Result<Self, ()> {
        let d3d9 = Direct3DCreate9(sdk_version.into());
        Rc::from_raw_opt(d3d9).ok_or(()).map(Self)
    }
}

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
        assert_eq!((), Direct3D::create(SdkVersion::from(0)).err().unwrap());
        assert_eq!((), Direct3D::create(SdkVersion::from(0).with_debug_disabled()).err().unwrap());
        assert_eq!((), Direct3D::create(SdkVersion::from(0).with_debug_enabled()).err().unwrap());
    }
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdepthstencilmatch)\]
    /// IDirect3D9::CheckDepthStencilMatch
    ///
    /// Determines whether a depth-stencil format is compatible with a render-target format in a particular display mode.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]   on most bad parameters
    /// *   <span class="inaccurate">[D3DERR::NOTAVAILABLE] "If a depth-stencil format is not compatible"</span>
    /// *   `Ok(())`                if the adapter x render target x depth stencil format combination is supported
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*;
    /// # let d3d = Direct3D::test();
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
    pub fn check_depth_stencil_match(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, adapter_format: impl Into<Format>, render_target_format: impl Into<Format>, depth_stencil_format: impl Into<Format>) -> Result<(), MethodError> {
        let hr = unsafe { self.0.CheckDepthStencilMatch(adapter, device_type.into().into(), adapter_format.into().into(), render_target_format.into().into(), depth_stencil_format.into().into()) };
        MethodError::check("IDirect3D9::CheckDepthStencilMatch", hr)?;
        Ok(())
    }
}

#[test] fn check_depth_stencil_match() {
    let d3d = Direct3D::test();
    assert!(                        d3d.check_depth_stencil_match(0,    DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, Format::D24S8  ).is_ok()); // valid
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(9001, DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, Format::D24S8  )); // invalid adapter
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(0,    Invalid,      Format::X8R8G8B8, Format::A8R8G8B8, Format::D24S8  )); // invalid devtype
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(0,    DevType::HAL, Format::UNKNOWN,  Format::A8R8G8B8, Format::D24S8  )); // invalid adapter format
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(0,    DevType::HAL, Format::X8R8G8B8, Format::UNKNOWN,  Format::D24S8  )); // invalid render target format
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_depth_stencil_match(0,    DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, Format::UNKNOWN)); // invalid depth stencil format
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdeviceformat)\]
    /// IDirect3D9::CheckDeviceFormat
    ///
    /// Determines whether a surface format is available as a specified resource type and can be used as a texture, depth-stencil buffer, or render target, or any combination of the three, on a device representing this adapter.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]       on most bad parameters
    /// *   <span class="inaccurate">[D3DERR::NOTAVAILABLE] "if the format is not acceptable to the device for this usage"</span>
    /// *   `Ok(())`                    if the format is supported
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// assert!(d3d.check_device_format(
    ///     D3DADAPTER_DEFAULT,
    ///     DevType::HAL,
    ///     Format::X8R8G8B8,       // adapter format
    ///     0,                      // usage
    ///     ResourceType::Texture,
    ///     Format::A8R8G8B8        // texture format
    /// ).is_ok());
    /// ```
    pub fn check_device_format(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, adapter_format: impl Into<Format>, usage: u32, rtype: impl Into<ResourceType>, check_format: impl Into<Format>) -> Result<(), MethodError> {
        let hr = unsafe { self.0.CheckDeviceFormat(adapter, device_type.into().into(), adapter_format.into().into(), usage, rtype.into().into(), check_format.into().into()) };
        MethodError::check("IDirect3D9::CheckDeviceFormat", hr)?;
        Ok(())
    }
}

#[test] fn check_device_format() {
    let d3d = Direct3D::test();
    assert!(                        d3d.check_device_format(0,    DevType::HAL, Format::X8R8G8B8,  0, ResourceType::Texture, Format::A8R8G8B8).is_ok()); // valid
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(9001, DevType::HAL, Format::X8R8G8B8,  0, ResourceType::Texture, Format::A8R8G8B8)); // invalid adapter
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    Invalid,      Format::X8R8G8B8,  0, ResourceType::Texture, Format::A8R8G8B8)); // invalid devtype
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    DevType::HAL, Format::UNKNOWN,   0, ResourceType::Texture, Format::A8R8G8B8)); // invalid adapter format
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    DevType::HAL, Format::X8R8G8B8, !0, ResourceType::Texture, Format::A8R8G8B8)); // invalid usage
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    DevType::HAL, Format::X8R8G8B8,  0, Invalid,               Format::A8R8G8B8)); // invalid resource type
    assert_eq!(D3DERR::INVALIDCALL, d3d.check_device_format(0,    DevType::HAL, Format::X8R8G8B8,  0, ResourceType::Texture, Format::UNKNOWN )); // invalid check format
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdeviceformatconversion)\]
    /// IDirect3D9::CheckDeviceFormatConversion
    ///
    /// Tests the device to see if it supports conversion from one display format to another.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDDEVICE]     if `device_type` is invalid
    /// *   [D3DERR::NOTAVAILABLE]      on an invalid source format
    /// *   [D3DERR::NOTAVAILABLE]      on an invalid target format
    /// *   [D3DERR::NOTAVAILABLE]      when the device cannot convert between the formats
    /// *   `Ok(())`                    if the device can convert from `source_format` to `target_format`
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// assert!(d3d.check_device_format_conversion(
    ///     D3DADAPTER_DEFAULT,
    ///     DevType::HAL,
    ///     Format::A8R8G8B8,
    ///     Format::A8R8G8B8
    /// ).is_ok());
    /// ```
    pub fn check_device_format_conversion(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, source_format: impl Into<Format>, target_format: impl Into<Format>) -> Result<(), MethodError> {
        let hr = unsafe { self.0.CheckDeviceFormatConversion(adapter, device_type.into().into(), source_format.into().into(), target_format.into().into()) };
        MethodError::check("IDirect3D9::CheckDeviceFormatConversion", hr)?;
        Ok(())
    }
}

#[test] fn check_device_format_conversion() {
    let d3d = Direct3D::test();
    assert!(                            d3d.check_device_format_conversion(0,    DevType::HAL, Format::A8R8G8B8, Format::A8R8G8B8).is_ok()); // valid
    assert!(                            d3d.check_device_format_conversion(0,    DevType::HAL, Format::D24S8,    Format::D24S8   ).is_ok()); // valid
    assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_format_conversion(9001, DevType::HAL, Format::A8R8G8B8, Format::A8R8G8B8)); // invalid adapter
    assert_eq!(D3DERR::INVALIDDEVICE,   d3d.check_device_format_conversion(0,    Invalid,      Format::A8R8G8B8, Format::A8R8G8B8)); // invalid devtype
    assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_format_conversion(0,    DevType::HAL, Invalid,          Format::A8R8G8B8)); // invalid source format
    assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_format_conversion(0,    DevType::HAL, Format::A8R8G8B8, Invalid         )); // invalid target format
    assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_format_conversion(0,    DevType::HAL, Format::D24S8,    Format::A8R8G8B8)); // invalid conversion
    assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_format_conversion(0,    DevType::HAL, Format::A8R8G8B8, Format::D24S8   )); // invalid conversion
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdevicemultisampletype)\]
    /// IDirect3D9::CheckDeviceMultiSampleType
    ///
    /// Tests the device to see if it supports conversion from one display format to another.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::NOTAVAILABLE]  on invalid surface formats, unless `D3DMULTISAMPLE_NONE`
    /// *   `Ok(quality_levels)`    if the multisampling type is valid for the given format
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// assert!(d3d.check_device_multi_sample_type(
    ///     D3DADAPTER_DEFAULT,
    ///     DevType::HAL,
    ///     Format::A8R8G8B8,
    ///     true,
    ///     MultiSample::None,
    /// ).is_ok());
    /// ```
    pub fn check_device_multi_sample_type(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, surface_format: impl Into<Format>, windowed: bool, multi_sample_type: impl Into<MultiSample>) -> Result<u32, MethodError> {
        let mut quality_levels = 0;
        let hr = unsafe { self.0.CheckDeviceMultiSampleType(adapter, device_type.into().into(), surface_format.into().into(), windowed.into(), multi_sample_type.into().into(), &mut quality_levels) };
        MethodError::check("IDirect3D9::CheckDeviceMultiSampleType", hr)?;
        Ok(quality_levels)
    }
}

#[test] fn check_device_multi_sample_type() {
    let d3d = Direct3D::test();
    assert!(                            d3d.check_device_multi_sample_type(0,    DevType::HAL, Format::A8R8G8B8, true, MultiSample::None    ).is_ok()); // valid
    assert!(                            d3d.check_device_multi_sample_type(0,    DevType::HAL, Format::A8R8G8B8, true, MultiSample::X2      ).is_ok()); // valid
    assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_multi_sample_type(9001, DevType::HAL, Format::A8R8G8B8, true, MultiSample::X2      )); // invalid adapter
    assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_multi_sample_type(0,    Invalid,      Format::A8R8G8B8, true, MultiSample::X2      )); // invalid devtype
    assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_multi_sample_type(0,    DevType::HAL, Invalid,          true, MultiSample::X2      )); // invalid surface format
    let _ =                             d3d.check_device_multi_sample_type(0,    DevType::HAL, Invalid,          true, MultiSample::None     ); // can succeed despite invalid format when we use D3DMULTISAMPLE_NONE
    assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_multi_sample_type(0,    DevType::HAL, Format::A8R8G8B8, true, Invalid              )); // invalid multisampling
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-checkdevicetype)\]
    /// IDirect3D9::CheckDeviceType
    ///
    /// Tests the device to see if it supports conversion from one display format to another.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]   if `device_type` is invalid
    /// *   [D3DERR::NOTAVAILABLE]  if `adapter_format` x `back_buffer_format` x `windowed` is invalid
    /// *   `Ok(())`                otherwise
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// assert!(d3d.check_device_type(
    ///     D3DADAPTER_DEFAULT, // adapter
    ///     DevType::HAL,       // device_type
    ///     Format::X8R8G8B8,   // adapter_format
    ///     Format::A8R8G8B8,   // back_buffer_format
    ///     true,               // windowed
    /// ).is_ok());
    /// ```
    pub fn check_device_type(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, adapter_format: impl Into<Format>, back_buffer_format: impl Into<Format>, windowed: bool) -> Result<(), MethodError> {
        let hr = unsafe { self.0.CheckDeviceType(adapter, device_type.into().into(), adapter_format.into().into(), back_buffer_format.into().into(), windowed.into()) };
        MethodError::check("IDirect3D9::CheckDeviceType", hr)?;
        Ok(())
    }
}

#[test] fn check_device_type() {
    let d3d = Direct3D::test();
    assert!(                            d3d.check_device_type(0,    DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, true).is_ok()); // valid
    assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_type(9001, DevType::HAL, Format::X8R8G8B8, Format::A8R8G8B8, true)); // invalid adapter
    assert_eq!(D3DERR::INVALIDCALL,     d3d.check_device_type(0,    Invalid,      Format::X8R8G8B8, Format::A8R8G8B8, true)); // invalid devtype
    assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_type(0,    DevType::HAL, Invalid,          Format::A8R8G8B8, true)); // invalid adapter format
    assert_eq!(D3DERR::NOTAVAILABLE,    d3d.check_device_type(0,    DevType::HAL, Format::X8R8G8B8, Invalid,          true)); // invalid back buffer format
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-createdevice)\]
    /// IDirect3D9::CreateDevice
    ///
    /// Creates a [Device].
    ///
    /// ### Safety
    ///
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
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// let device = unsafe { d3d.create_device(
    ///     D3DADAPTER_DEFAULT,     // adapter
    ///     DevType::HAL,           // device_type
    ///     null_mut(),             // focus_window
    ///     D3DCREATE_FPU_PRESERVE, // behavior_flags
    ///     &mut D3DPRESENT_PARAMETERS {
    ///         // In/Out paramters - if these are 0 before the method create_device
    ///         // is called, they will be changed when create_device returns
    ///         BackBufferWidth:            0,
    ///         BackBufferHeight:           0,
    ///         BackBufferCount:            0,
    ///
    ///         // In/Out - if this equals Format::UNKNOWN it will be changed when
    ///         // create_device returns
    ///         BackBufferFormat:           Format::X8R8G8B8.into(),
    ///
    ///         // In parameters
    ///         MultiSampleType:            MultiSample::None.into(),
    ///         MultiSampleQuality:         0,
    ///         SwapEffect:                 D3DSWAPEFFECT_DISCARD,
    ///         hDeviceWindow:              null_mut(),
    ///         Windowed:                   true.into(),
    ///         EnableAutoDepthStencil:     false.into(),
    ///         AutoDepthStencilFormat:     Format::UNKNOWN.into(),
    ///         Flags:                      0,
    ///         FullScreen_RefreshRateInHz: 0,
    ///         PresentationInterval:       0,
    ///     }
    /// )};
    /// let _device = device; // XXX
    /// ```
    ///
    /// [WM_DESTROY]:           https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy
    pub unsafe fn create_device(&self, adapter: AdapterIndex, device_type: impl Into<DevType>, focus_window: HWND, behavior_flags: u32, present_parameters: &mut D3DPRESENT_PARAMETERS) -> Result<Device, MethodError> {
        // TODO: better doc comments
        let mut device = null_mut();
        let hr = self.0.CreateDevice(adapter, device_type.into().into(), focus_window, behavior_flags, present_parameters, &mut device);
        MethodError::check("IDirect3D9::CreateDevice", hr)?;
        Ok(Device::from_raw(device))
    }
}

// #[test] fn create_device() {} // TODO



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-enumadaptermodes)\] IDirect3D9::EnumAdapterModes
    ///
    /// Queries the possible display modes of an adapter (~ connected monitor)
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]       if `mode` >= `self.get_adapter_mode_count(adapter, format)`
    /// *   Ok([D3DDISPLAYMODE])        otherwise
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// let adapter = 0;
    /// let fmt = Format::X8R8G8B8;
    /// for mode in 0..d3d.get_adapter_mode_count(adapter, fmt) {
    ///     let mode = d3d.enum_adapter_modes(adapter, fmt, mode).unwrap();
    ///     let D3DDISPLAYMODE { Width: w, Height: h, RefreshRate: hz, Format: fmt2 } = mode;
    ///     println!("{}x{} @ {}hz {:?}", w, h, hz, Format::from_unchecked(fmt2));
    /// }
    /// ```
    ///
    /// ### Output
    ///
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
    pub fn enum_adapter_modes(&self, adapter: AdapterIndex, format: impl Into<Format>, mode: ModeIndex) -> Result<D3DDISPLAYMODE, MethodError> {
        let mut dm = unsafe { std::mem::zeroed::<D3DDISPLAYMODE>() };
        let hr = unsafe { self.0.EnumAdapterModes(adapter, format.into().into(), mode, &mut dm) };
        MethodError::check("IDirect3D9::EnumAdapterModes", hr)?;
        Ok(dm)
    }
}

#[test] fn enum_adapter_modes() {
    let d3d = Direct3D::test();
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
                assert_eq!(D3DERR::INVALIDCALL, d3d.enum_adapter_modes(adapter, fmt, mode).map(|_| ()));
            }
        }
    }

    // doublecheck that d3d doesn't segfault for out-of-bounds adapters or anything
    for adapter in [adapters+0, adapters+100, adapters+100000, adapters+10000000].iter().copied() {
        eprintln!("checking invalid adapter {} of {}", adapter+1, adapters);
        assert_eq!(D3DERR::INVALIDCALL, d3d.enum_adapter_modes(adapter, Format::X8R8G8B8, 0).map(|_| ()));
        assert_eq!(D3DERR::INVALIDCALL, d3d.enum_adapter_modes(adapter, Format::X8R8G8B8, 100).map(|_| ()));
        assert_eq!(D3DERR::INVALIDCALL, d3d.enum_adapter_modes(adapter, Format::X8R8G8B8, 10000).map(|_| ()));
    }
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadaptercount)\] IDirect3D9::GetAdapterCount
    ///
    /// Gets the number of adapters (~ connected monitors) available to this device.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// println!("{} adapters", d3d.get_adapter_count());
    /// ```
    ///
    /// ### Output
    ///
    /// ```text
    /// 4 adapters
    /// ```
    pub fn get_adapter_count(&self) -> AdapterIndex {
        unsafe { self.0.GetAdapterCount() } // Safety:  Seems 100% safe per unit testing bellow
    }
}

#[test] fn get_adapter_count() {
    let d3d = Direct3D::test();
    assert!(d3d.get_adapter_count() > 0);
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadapterdisplaymode)\] IDirect3D9::GetAdapterDisplayMode
    ///
    /// Gets the current display mode of an adapter (~ connected monitor)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// for adapter in 0..d3d.get_adapter_count() {
    ///     let mode = d3d.get_adapter_display_mode(adapter).unwrap();
    ///     let D3DDISPLAYMODE { Width: w, Height: h, RefreshRate: hz, Format: fmt } = mode;
    ///     println!("{}x{} @ {}hz {:?}", w, h, hz, Format::from_unchecked(fmt));
    /// }
    /// ```
    ///
    /// ### Output
    ///
    /// ```text
    /// 2160x3840 @ 60hz Format(D3DFMT_X8R8G8B8)
    /// 2160x3840 @ 60hz Format(D3DFMT_X8R8G8B8)
    /// 2160x3840 @ 29hz Format(D3DFMT_X8R8G8B8)
    /// 2160x3840 @ 29hz Format(D3DFMT_X8R8G8B8)
    /// ```
    pub fn get_adapter_display_mode(&self, adapter: AdapterIndex) -> Result<D3DDISPLAYMODE, MethodError> {
        let mut dm = unsafe { std::mem::zeroed::<D3DDISPLAYMODE>() };
        let hr = unsafe { self.0.GetAdapterDisplayMode(adapter, &mut dm) };
        MethodError::check("IDirect3D9::GetAdapterDisplayMode", hr)?;
        Ok(dm)
    }
}

#[test] fn get_adapter_display_mode() {
    let d3d = Direct3D::test();
    let adapters = d3d.get_adapter_count();
    for adapter in 0..adapters {
        d3d.get_adapter_display_mode(adapter).unwrap_or_else(|err| panic!("unable to query display mode of adapter {} of {}: {}", adapter+1, adapters, err));
    }
    for adapter in adapters..(100+adapters) {
        assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_display_mode(adapter).err());
    }
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadapteridentifier)\] IDirect3D9::GetAdapterIdentifier
    ///
    /// Gets metadata about an adapter (~ connected monitor), including driver name/version/guids/whql info/vendor/device ids/blood type/birthplace/???
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `adapter` >= `self.get_adapter_count()`
    /// *   [D3DERR::INVALIDCALL]       if `flags` isn't valid (0, D3DENUM_WHQL_LEVEL, ...)
    /// *   Ok([AdapterIdentifier])     otherwise
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// eprintln!("{:#?}", d3d.get_adapter_identifier(0, 0).unwrap());
    /// ```
    ///
    /// ### Output
    ///
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
    pub fn get_adapter_identifier(&self, adapter: AdapterIndex, flags: u32) -> Result<AdapterIdentifier, MethodError> {
        let mut ident = AdapterIdentifier::default();
        let hr = unsafe { self.0.GetAdapterIdentifier(adapter, flags, &mut *ident) };
        MethodError::check("IDirect3D9::GetAdapterIdentifier", hr)?;
        Ok(ident)
    }
}

#[test] fn get_adapter_identifier() {
    let d3d = Direct3D::test();

    /// The D3DENUM_WHQL_LEVEL value has been retired for 9Ex and future versions
    ///
    /// Ref: `d3d9.h`
    const D3DENUM_WHQL_LEVEL : u32 = 2;

    let valid = 0;
    d3d.get_adapter_identifier(valid, 0                 ).unwrap();
    d3d.get_adapter_identifier(valid, D3DENUM_WHQL_LEVEL).unwrap();
    assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(valid, D3DENUM_WHQL_LEVEL+1).map(|_| ()));
    assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(valid, D3DENUM_WHQL_LEVEL+1000).map(|_| ()));
    assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(valid, D3DENUM_WHQL_LEVEL+1000000).map(|_| ()));

    let invalid = 9001;
    assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(invalid, 0                   ).map(|_| ()));
    assert_eq!(D3DERR::INVALIDCALL, d3d.get_adapter_identifier(invalid, D3DENUM_WHQL_LEVEL  ).map(|_| ()));
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadaptermodecount)\] IDirect3D9::GetAdapterModeCount
    ///
    /// ### Arguments
    ///
    /// * `adapter` - the adapter/monitor to query modes for
    /// * `format` - the adapter format to query modes for.  `Format::X8R8G8B8` likely works, many other formats likely don't.
    ///
    /// ### Returns
    ///
    /// *   `0`         if `adapter` >= `self.get_adapter_count()`
    /// *   `0`         if `format` is an invalid format
    /// *   `0`         if `format` is an unsupported format
    /// *   `n`         modes you can query [enum_adapter_modes] for
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// for fmt in [Format::X8R8G8B8, Format::A8R8G8B8].iter().copied() {
    ///     let modes = d3d.get_adapter_mode_count(0, fmt);
    ///     println!("adapter 1: {:?}: {} mode(s)", fmt, modes);
    /// }
    /// ```
    ///
    /// ### Output
    ///
    /// ```text
    /// adapter 1: Format(D3DFMT_X8R8G8B8): 44 mode(s)
    /// adapter 1: Format(D3DFMT_A8R8G8B8): 0 mode(s)
    /// ```
    ///
    /// ### See Also
    ///
    /// * [enum_adapter_modes]
    ///
    /// [enum_adapter_modes]:       #method.enum_adapter_modes
    pub fn get_adapter_mode_count(&self, adapter: AdapterIndex, format: impl Into<Format>) -> ModeIndex {
        unsafe { self.0.GetAdapterModeCount(adapter, format.into().into()) }
    }
}

#[test] fn get_adapter_mode_count() {
    let d3d = Direct3D::test();
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



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getadaptermonitor)\] IDirect3D9::GetAdapterMonitor
    ///
    /// ### Returns
    ///
    /// *   `Err(())`               on an invalid `adapter`
    /// *   Ok([HMONITOR])          otherwise
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// let monitor : HMONITOR = d3d.get_adapter_monitor(   0).unwrap();
    /// let error   : ()       = d3d.get_adapter_monitor(9001).unwrap_err();
    /// ```
    pub fn get_adapter_monitor(&self, adapter: AdapterIndex) -> Result<HMONITOR, ()> {
        let hm = unsafe { self.0.GetAdapterMonitor(adapter) }; // Safety: Seems to be safe even when `adapter` >= `self.get_adapter_count()` per unit test bellow
        if hm.is_null() { Err(()) } else { Ok(hm) }
    }
}
#[test] fn get_adapter_monitor() {
    let d3d = Direct3D::test();
    let adapters = d3d.get_adapter_count();
    for valid in 0..adapters                { assert!(!d3d.get_adapter_monitor(  valid).unwrap_or(null_mut()).is_null(), "adapter {} of {} has a null HMONITOR", valid+1, adapters); }
    for invalid in adapters..adapters+100   { assert!( d3d.get_adapter_monitor(invalid).is_err(),                        "adapter {} of {} returned an HMONITOR despite being out-of-bounds!", invalid+1, adapters); }
}



impl Direct3D {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-getdevicecaps)\] IDirect3D9::GetDeviceCaps
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       on an invalid `adapter`
    /// *   [D3DERR::INVALIDDEVICE]     on an invalid `device_type`
    /// *   Ok([Caps])                  otherwise
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let d3d = Direct3D::test();
    /// let caps : Caps = d3d.get_device_caps(0, DevType::HAL).unwrap();
    /// assert_eq!(caps.DeviceType,     DevType::HAL.into());
    /// assert_eq!(caps.AdapterOrdinal, 0);
    /// assert!(caps.MaxTextureWidth  > 0);
    /// assert!(caps.MaxTextureHeight > 0);
    /// // ...
    /// ```
    pub fn get_device_caps(&self, adapter: AdapterIndex, device_type: DevType) -> Result<Caps, MethodError> {
        let mut caps = Caps::default();
        let hr = unsafe { self.0.GetDeviceCaps(adapter, device_type.into().into(), &mut *caps) }; // Safety: Appears sound on all invalid parameters per unit testing
        MethodError::check("IDirect3D9::GetDeviceCaps", hr)?;
        Ok(caps)
    }
}

#[test] fn get_device_caps() {
    let d3d = Direct3D::test();

    let adapter = 0; // valid
    d3d.get_device_caps(adapter, DevType::HAL).unwrap();
    d3d.get_device_caps(adapter, DevType::Ref).unwrap();
    d3d.get_device_caps(adapter, DevType::NullRef).unwrap();
    assert_eq!(D3DERR::INVALIDDEVICE,   d3d.get_device_caps(adapter, DevType::from_unchecked(100)).map(|_| ()));
    assert_eq!(D3DERR::INVALIDDEVICE,   d3d.get_device_caps(adapter, DevType::from_unchecked(10000)).map(|_| ()));
    assert_eq!(D3DERR::INVALIDDEVICE,   d3d.get_device_caps(adapter, DevType::from_unchecked(1000000)).map(|_| ()));
    assert_eq!(D3DERR::INVALIDDEVICE,   d3d.get_device_caps(adapter, DevType::from_unchecked(100000000)).map(|_| ()));

    let adapter = 9001; // invalid
    assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::HAL).map(|_| ()));
    assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::Ref).map(|_| ()));
    assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::NullRef).map(|_| ()));
    assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::from_unchecked(100)).map(|_| ()));
    assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::from_unchecked(10000)).map(|_| ()));
    assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::from_unchecked(1000000)).map(|_| ()));
    assert_eq!(D3DERR::INVALIDCALL,     d3d.get_device_caps(adapter, DevType::from_unchecked(100000000)).map(|_| ()));
}



impl Direct3D {
    // XXX: register_software_device
    // I can't easily find proper documentation demonstrating usage.  Supposedly it's buried in a DDK somewhere.
    // Pull requests welcome w/ unit testing coverage!
}



#[cfg(test)] impl Direct3D {
    pub fn test() -> Self {
        unsafe { Direct3D::create(SdkVersion::default()).unwrap() }
    }
}

#[cfg(test)] pub(crate) struct Invalid;
#[cfg(test)] impl From<Invalid> for DevType         { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
#[cfg(test)] impl From<Invalid> for Format          { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
#[cfg(test)] impl From<Invalid> for FVF             { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
#[cfg(test)] impl From<Invalid> for MultiSample     { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
#[cfg(test)] impl From<Invalid> for Pool            { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
#[cfg(test)] impl From<Invalid> for ResourceType    { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
#[cfg(test)] impl From<Invalid> for Usage           { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
