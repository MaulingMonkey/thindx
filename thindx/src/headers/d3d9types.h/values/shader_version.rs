use bytemuck::*;
use winapi::shared::minwindef::DWORD;
use std::fmt::{self, Debug, Formatter};



#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)]
#[repr(transparent)]
pub struct ShaderVersion(DWORD);

impl ShaderVersion {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dps-version)\]
    /// D3DPS_VERSION
    pub const fn ps(major: u8, minor: u8) -> Self {
        Self(0xFFFF0000 | ((major as u32) << 8) | ((minor as u32) << 0))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvs-version)\]
    /// D3DVS_VERSION
    pub const fn vs(major: u8, minor: u8) -> Self {
        Self(0xFFFE0000 | ((major as u32) << 8) | ((minor as u32) << 0))
    }

    /// \[docs.microsoft.com\]
    /// D3DSHADER_VERSION_MAJOR
    pub fn version_major(&self) -> u8 { (self.0 >> 8) as _ }

    /// \[docs.microsoft.com\]
    /// D3DSHADER_VERSION_MINOR
    pub fn version_minor(&self) -> u8 { (self.0 >> 0) as _ }



    // all valid pixel shader versions per
    // <https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dps-version#remarks>

    pub const PS_1_1 : ShaderVersion = ShaderVersion::ps(1, 1);
    pub const PS_1_2 : ShaderVersion = ShaderVersion::ps(1, 2);
    pub const PS_1_3 : ShaderVersion = ShaderVersion::ps(1, 3);
    pub const PS_1_4 : ShaderVersion = ShaderVersion::ps(1, 4);
    pub const PS_2_0 : ShaderVersion = ShaderVersion::ps(2, 0);
    pub const PS_3_0 : ShaderVersion = ShaderVersion::ps(2, 0);

    // all valid vertex shader versions per
    // <https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvs-version#remarks>

    pub const VS_1_1 : ShaderVersion = ShaderVersion::vs(1, 1);
    pub const VS_2_0 : ShaderVersion = ShaderVersion::vs(1, 0);
    pub const VS_3_0 : ShaderVersion = ShaderVersion::vs(1, 0);
}

impl Debug for ShaderVersion {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let pre = match self.0 >> 16 {
            0xFFFF => "PS",
            0xFFFE => "VS",
            _other => return write!(fmt, "ShaderVersion(0x{:08x})", self.0),
        };
        write!(fmt, "ShaderVersion::{}_{}_{}", pre, self.version_major(), self.version_minor())
    }
}

//#cpp2rust D3DPS_VERSION           = d3d9::ShaderVersion::ps
//#cpp2rust D3DVS_VERSION           = d3d9::ShaderVersion::vs
//#cpp2rust D3DSHADER_VERSION_MAJOR = d3d9::ShaderVersion::version_major
//#cpp2rust D3DSHADER_VERSION_MINOR = d3d9::ShaderVersion::version_minor
