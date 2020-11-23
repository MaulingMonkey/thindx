#![allow(dead_code)] // TODO: remove

use crate::*;

use winapi::shared::d3d9types::*;

use std::ptr::null_mut;



/// # Miscellanious
/// Metadata, etc.
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-evictmanagedresources)]\
    /// IDirect3DDevice9::EvictManagedResources
    ///
    /// Evicts all managed resources, including both Direct3D and driver-managed resources.
    ///
    /// This function causes only the [Pool::Default] copy of resources to be evicted.
    /// The resource copy in system memory is retained. See [Pool].
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::COMMAND_UNPARSED]
    /// *   Ok(())
    pub(crate) fn evict_managed_resources(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.0.EvictManagedResources() };
        MethodError::check("IDirect3DDevice9::EvictManagedResources", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getavailabletexturemem)\]
    /// IDirect3DDevice9::GetAvailableTextureMem
    ///
    /// Returns an estimate of the amount of available texture memory.
    ///
    /// The returned value is rounded to the nearest MB.
    /// This is done to reflect the fact that video memory estimates are never precise due to alignment and other issues that affect consumption by certain resources.
    /// Applications can use this value to make gross estimates of memory availability to make large-scale resource decisions such as how many levels of a mipmap to attempt to allocate,
    /// but applications cannot use this value to make small-scale decisions such as if there is enough memory left to allocate another resource.
    ///
    /// ### Returns
    ///
    /// *   `0xFFE00000`
    /// *   Maybe occasionally some other values too
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let available = device.get_available_texture_mem();
    /// if available >= 0xFFE0_0000 {
    ///     println!("> 4 GiB available");
    /// } else {
    ///     println!("~ {} MiB available", available / 1024 / 1024);
    /// }
    /// ```
    ///
    /// ### Output
    ///
    /// ```text
    /// > 4 GiB available
    /// ```
    pub fn get_available_texture_mem(&self) -> u32 {
        unsafe { self.0.GetAvailableTextureMem() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipplane)\]
    /// IDirect3DDevice9::GetClipPlane
    ///
    /// Retrieves the coefficients of a user-defined clipping plane for the device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the index exceeds the maximum clipping pane index supported by the device (if there is such a limit)
    /// *   `Ok([A, B, C, D])`, where points `Ax + By + Cz + Dw >= 0` are visible
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// println!("{:?}", device.get_clip_plane(0).unwrap());
    /// ```
    ///
    /// ### Output
    ///
    /// ```text
    /// [0.0, 0.0, 0.0, 0.0]
    /// ```
    pub fn get_clip_plane(&self, index: u32) -> Result<[f32; 4], MethodError> {
        let mut plane = [0.0, 0.0, 0.0, 0.0];
        let hr = unsafe { self.0.GetClipPlane(index, plane.as_mut_ptr()) };
        MethodError::check("IDirect3DDevice9::GetClipPlane", hr)?;
        Ok(plane)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipstatus)\]
    /// IDirect3DDevice9::GetClipStatus
    ///
    /// Retrieves the clip status.
    ///
    /// ### Returns
    ///
    /// *   <span class="inaccurate">[D3DERR::INVALIDCALL]  - "if the argument is invalid", but this should always be valid"
    /// *   Ok([ClipStatus])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// println!("{:?}", device.get_clip_status().unwrap());
    /// ```
    ///
    /// ### Output
    ///
    /// ```text
    /// ClipStatus { ClipUnion: 0, ClipIntersection: 4294967295 }
    /// ```
    pub fn get_clip_status(&self) -> Result<ClipStatus, MethodError> {
        let mut status = D3DCLIPSTATUS9 { ClipUnion: 0, ClipIntersection: 0 };
        let hr = unsafe { self.0.GetClipStatus(&mut status) };
        MethodError::check("IDirect3DDevice9::GetClipStatus", hr)?;
        Ok(ClipStatus::from_unchecked(status))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getcreationparameters)\]
    /// IDirect3DDevice9::GetCreationParameters
    ///
    /// Retrieves the creation parameters of the device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   "If the returned argument is invalid" (impossible via thin3d9?)
    /// *   Ok(())
    pub fn get_creation_parameters(&self) -> Result<D3DDEVICE_CREATION_PARAMETERS, MethodError> {
        let mut dcp = unsafe { std::mem::zeroed::<D3DDEVICE_CREATION_PARAMETERS>() };
        let hr = unsafe { self.0.GetCreationParameters(&mut dcp) };
        MethodError::check("IDirect3DDevice9::GetCreationParameters", hr)?;
        Ok(dcp)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getcurrenttexturepalette)\]
    /// IDirect3DDevice9::GetCurrentTexturePalette
    ///
    /// Retrieves the current texture palette
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   "If the method fails" (impossible via thin3d9?)
    /// *   Ok(`texture_palette_index`)
    pub fn get_current_texture_palette(&self) -> Result<u32, MethodError> {
        let mut pal = 0;
        let hr = unsafe { self.0.GetCurrentTexturePalette(&mut pal) };
        MethodError::check("IDirect3DDevice9::GetCurrentTexturePalette", hr)?;
        Ok(pal)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdevicecaps)\]
    /// IDirect3DDevice9::GetDeviceCaps
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thin3d9?)
    /// *   Ok([Caps])                  otherwise
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let caps : Caps = device.get_device_caps().unwrap();
    /// assert_eq!(caps.DeviceType,     DevType::HAL.into());
    /// assert_eq!(caps.AdapterOrdinal, 0);
    /// assert!(caps.MaxTextureWidth  > 0);
    /// assert!(caps.MaxTextureHeight > 0);
    /// // ...
    /// ```
    pub fn get_device_caps(&self) -> Result<Caps, MethodError> {
        let mut caps = Caps::default();
        let hr = unsafe { self.0.GetDeviceCaps(&mut *caps) };
        MethodError::check("IDirect3DDevice9::GetDeviceCaps", hr)?;
        Ok(caps)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdirect3d)\]
    /// IDirect3DDevice9::GetDirect3D
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thin3d9?)
    /// *   Ok([Direct3D])              otherwise
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let d3d : Direct3D = device.get_direct3d().unwrap();
    /// ```
    pub fn get_direct3d(&self) -> Result<Direct3D, MethodError> {
        let mut d3d = null_mut();
        let hr = unsafe { self.0.GetDirect3D(&mut d3d) };
        MethodError::check("IDirect3DDevice9::GetDirect3D", hr)?;
        Ok(unsafe { Direct3D::from_raw(d3d) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdisplaymode)\]
    /// IDirect3DDevice9::GetDisplayMode
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thin3d9?)
    /// *   Ok([DisplayMode])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let mode : DisplayMode = device.get_display_mode(0).unwrap();
    /// println!("{:#?}", mode);
    /// assert!(mode.Width > 0);
    /// assert!(mode.width() > 0);
    /// assert_eq!(mode.Format, Format::X8R8G8B8.into());
    /// assert_eq!(mode.format(), Format::X8R8G8B8);
    /// ```
    ///
    /// ### Outputs
    ///
    /// ```text
    /// DisplayMode {
    ///     Width: 2160,
    ///     Height: 3840,
    ///     RefreshRate: 60,
    ///     Format: Format(D3DFMT_X8R8G8B8),
    /// }
    /// ```
    pub fn get_display_mode(&self, swap_chain: u32) -> Result<DisplayMode, MethodError> {
        let mut dm = DisplayMode::default();
        let hr = unsafe { self.0.GetDisplayMode(swap_chain, &mut *dm) };
        MethodError::check("IDirect3DDevice9::GetDisplayMode", hr)?;
        Ok(dm)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getfvf)\]
    /// IDirect3DDevice9::GetFVF
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thin3d9?)
    /// *   Ok([FVF])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// assert_eq!(device.get_fvf().unwrap(), FVF::None);
    /// ```
    pub fn get_fvf(&self) -> Result<FVF, MethodError> {
        let mut fvf = FVF::None;
        let hr = unsafe { self.0.GetFVF(&mut *fvf) };
        MethodError::check("IDirect3DDevice9::GetFVF", hr)?;
        Ok(fvf)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getgammaramp)\]
    /// IDirect3DDevice9::GetGammaRamp
    ///
    /// ### Returns
    ///
    /// *   [D3DGAMMARAMP]
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let ramp = device.get_gamma_ramp(0);
    /// ```
    pub fn get_gamma_ramp(&self, swap_chain: u32) -> D3DGAMMARAMP {
        let mut ramp = unsafe { std::mem::zeroed::<D3DGAMMARAMP>() };
        let _nohr : () = unsafe { self.0.GetGammaRamp(swap_chain, &mut ramp) };
        ramp
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlight)\]
    /// IDirect3DDevice9::GetLight
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   - if no light was previously set at `index`
    /// *   Ok([Light])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light(0));
    ///
    /// let mut light = Light::default();
    /// light.Type = LightType::Point.into();
    /// // ...
    /// device.set_light(0, light).unwrap();
    ///
    /// let light = device.get_light(0).unwrap();
    /// ```
    pub fn get_light(&self, index: u32) -> Result<Light, MethodError> {
        let mut light = Light::default();
        let hr = unsafe { self.0.GetLight(index, &mut *light) };
        MethodError::check("IDirect3DDevice9::GetLight", hr)?;
        Ok(light)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlightenable)
    /// IDirect3DDevice9::GetLightEnable
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the light was never explicitly previously enabled or disabled
    /// *   [D3DERR::INVALIDCALL]       The device is a pure device?
    /// *   Ok(`true`)                  The light is enabled
    /// *   Ok(`false`)                 The light is disabled
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable(0));
    ///
    /// device.light_enable(0, false).unwrap();
    ///
    /// let enabled0 = device.get_light_enable(0).unwrap();
    /// assert_eq!(enabled0, false);
    /// ```
    pub fn get_light_enable(&self, index: u32) -> Result<bool, MethodError> {
        let mut enable = 0;
        let hr = unsafe { self.0.GetLightEnable(index, &mut enable) };
        MethodError::check("IDirect3DDevice9::GetLightEnable", hr)?;
        Ok(enable != 0)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getmaterial)\]
    /// IDirect3DDevice9::GetMaterial
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the material is invalid
    /// *   Ok([Material])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let material = device.get_material().unwrap();
    /// ```
    pub fn get_material(&self) -> Result<Material, MethodError> {
        let mut material = Material::default();
        let hr = unsafe { self.0.GetMaterial(&mut *material) };
        MethodError::check("IDirect3DDevice9::GetMaterial", hr)?;
        Ok(material)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getnpatchmode)\]
    /// IDirect3DDevice9::GetNPatchMode
    ///
    /// Gets the N-patch mode segments.
    ///
    /// Specifies the number of subdivision segments.
    /// If the number of segments is less than 1.0, N-patches are disabled.
    /// The default value is 0.0.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// assert_eq!(device.get_npatch_mode(), 0.0);
    /// ```
    pub fn get_npatch_mode(&self) -> f32 {
        unsafe { self.0.GetNPatchMode() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setfvf)\]
    /// IDirect3DDevice9::SetFVF
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails (impossible via thin3d9?)
    /// *   Ok(())
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// device.set_fvf(FVF::None).unwrap();
    /// device.set_fvf(FVF::XYZ).unwrap();
    /// ```
    pub fn set_fvf(&self, fvf: impl Into<FVF>) -> Result<(), MethodError> {
        let hr = unsafe { self.0.SetFVF(fvf.into().into()) };
        MethodError::check("IDirect3DDevice9::SetFVF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setgammaramp)\]
    /// IDirect3DDevice9::SetGammaRamp
    ///
    /// Sets the gamma correction ramp for the implicit swap chain. This method will affect the entire screen (not just the active window if you are running in windowed mode).
    ///
    /// If the device does not support gamma ramps in the swap chain's current presentation mode (full-screen or windowed), no error return is given.
    /// Applications can check the D3DCAPS2_FULLSCREENGAMMA and D3DCAPS2_CANCALIBRATEGAMMA capability bits in the Caps2 member of the D3DCAPS9 structure to determine the capabilities of the device and whether a calibrator is installed.
    ///
    /// For windowed gamma correction presentation, use [SwapChain::present] if the hardware supports the feature.
    /// In DirectX 8, SetGammaRamp will set the gamma ramp only on a full-screen mode application.
    /// For more information about gamma correction, see [Gamma (Direct3D 9)].
    ///
    /// ### Returns
    ///
    /// *   `()`
    ///
    /// ### Example
    ///
    /// ```rust,no_run
    /// # use doc::*; let device = Device::test();
    /// let ramp = device.get_gamma_ramp(0);
    /// // ...modify ramp?..
    /// device.set_gamma_ramp(0, SGR::NoCalibration, &ramp);
    /// ```
    ///
    /// [Gamma (Direct3D 9)]:           https://docs.microsoft.com/en-us/windows/desktop/direct3d9/gamma
    pub fn set_gamma_ramp(&self, swap_chain: u32, flags: impl Into<SGR>, ramp: &D3DGAMMARAMP) {
        let _nohr : () = unsafe { self.0.SetGammaRamp(swap_chain, flags.into().into(), ramp) };
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setlight)\]
    /// IDirect3DDevice9::SetLight
    ///
    /// Assigns a set of lighting properties for this device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust,no_run
    /// # use doc::*; let device = Device::test();
    /// let mut light = Light::default();
    /// light.Type = LightType::Point.into();
    /// // ...
    /// device.set_light(0, light).unwrap();
    /// ```
    pub fn set_light(&self, index: u32, light: impl Into<Light>) -> Result<(), MethodError> {
        let light = light.into();
        let hr = unsafe { self.0.SetLight(index, &*light) };
        MethodError::check("IDirect3DDevice9::SetLight", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-lightenable)\]
    /// IDirect3DDevice9::LightEnable
    ///
    /// Enables or disables a set of lighting parameters within a device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust,no_run
    /// # use doc::*; let device = Device::test();
    /// device.light_enable(0, true).unwrap();
    /// device.light_enable(0, false).unwrap();
    /// ```
    pub fn light_enable(&self, index: u32, enable: bool) -> Result<(), MethodError> {
        let hr = unsafe { self.0.LightEnable(index, enable.into()) };
        MethodError::check("IDirect3DDevice9::LightEnable", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setmaterial)\]
    /// IDirect3DDevice9::SetMaterial
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the material is invalid
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let material = Material {
    ///     diffuse: ColorValue::default(),
    ///     .. Material::default()
    /// };
    /// device.set_material(material).unwrap();
    /// ```
    pub fn set_material(&self, material: impl Into<Material>) -> Result<(), MethodError> {
        let hr = unsafe { self.0.SetMaterial(&*material.into()) };
        MethodError::check("IDirect3DDevice9::SetMaterial", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getnpatchmode)\]
    /// IDirect3DDevice9::SetNPatchMode
    ///
    /// Specifies the number of subdivision segments.
    /// If the number of segments is less than 1.0, N-patches are disabled.
    /// The default value is 0.0.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// device.set_npatch_mode(0.0).unwrap();
    /// device.set_npatch_mode(1.0).unwrap();
    /// device.set_npatch_mode(9001.0).unwrap();
    /// ```
    pub fn set_npatch_mode(&self, mode: f32) -> Result<(), MethodError> {
        let hr = unsafe { self.0.SetNPatchMode(mode) };
        MethodError::check("IDirect3DDevice9::SetNPatchMode", hr)
    }
}

#[test] fn evict_managed_resources() {
    let device = Device::test();
    for _ in 0..1000 { device.evict_managed_resources().unwrap(); }
    // TODO: Create some Pool::Default and Pool::Managed resources
    // as I understand it, this will only evict cached copies of the latter which is only a perf thing
}

#[test] fn get_available_texture_mem() {
    let device = Device::test();
    let available = device.get_available_texture_mem();
    assert!(available >= 1024 * 1024 * 1024); // probably a bug if our modern computer doesn't have at least 1 GiB of video mem available
}

#[test] fn get_clip_plane() {
    let device  = Device::test();
    let _plane0 = device.get_clip_plane(0).unwrap();
    let _planen = device.get_clip_plane(!0).unwrap(); // never fails?
}

#[test] fn get_clip_status() {
    let device = Device::test();
    let _status = device.get_clip_status().unwrap(); // never fails?
}

#[test] fn get_creation_parameters() {
    let device = Device::test();
    let _dcp = device.get_creation_parameters().unwrap(); // never fails?
}

#[test] fn get_current_texture_palette() {
    let device = Device::test();
    let pal = device.get_current_texture_palette().unwrap(); // never fails?
    assert!(pal == 0 || pal == 0xFFFF); // 0xFFFF on my machine - some specific "no palette" constant?
}
