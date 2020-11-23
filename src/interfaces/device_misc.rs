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
