#![allow(dead_code)] // TODO: remove

use crate::*;



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
    /// ```rust,no_run
    /// # use thin3d9::*;
    /// # let device : Device = unimplemented!();
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
