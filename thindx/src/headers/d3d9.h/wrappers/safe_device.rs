use crate::Error;
use crate::d3d9::*;

use std::ops::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9)\]
/// [Device] + stored metadata allowing more APIs to be safe
pub struct SafeDevice {
    device:     Device,
    caps:       Caps,
}

impl SafeDevice {
    /// Wrap & make a given [Device] "Safe"
    ///
    /// This may involve setting state on the device, including:
    /// 1.  Clearing all bound Textures
    /// 2.  Clearing all bound Palettes
    ///
    /// This list list should not be assumed to be exaustive, and will likely continue to be extended.
    ///
    /// To avoid causing new bugs from clearing state, you should generally construct this wrapper as early as possible.
    pub fn new(device: Device) -> Result<Self, Error> {
        let caps = device.get_device_caps()?;
        for t in 0 .. caps.MaxSimultaneousTextures {
            // SOUND1: get_texture is unsound unless set_texture is called first
            unsafe { device.set_texture(t, None) }?;
        }
        Ok(Self { device, caps })
    }

    pub(crate) fn device(&self) -> &Device { &self.device }
    pub(crate) fn caps(&self) -> &Caps { &self.caps }
}

impl Deref for SafeDevice {
    type Target = Device;
    fn deref(&self) -> &Self::Target { self.device() }
}
