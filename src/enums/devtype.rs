#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddevtype)\]
/// D3DDEVTYPE
///
/// All methods of the [Direct3D] interface that take a [DevType] will fail if [DevType::NullRef] is specified. To use these methods, substitute [DevType::Ref] in the method call.
///
/// A [DevType::Ref] device should be created in [Pool::Scratch] memory, unless vertex and index buffers are required. To support vertex and index buffers, create the device in [Pool::SystemMem] memory.
///
/// If D3dref9.dll is installed, Direct3D will use the reference rasterizer to create a [DevType::Ref] device type, even if [DevType::NullRef] is specified. If D3dref9.dll is not available and [DevType::NullRef] is specified, Direct3D will neither render nor present the scene.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DevType(D3DDEVTYPE);

impl DevType {
    /// Convert a raw [D3DDEVTYPE] value into a [DevType].  This is *probably* safe... probably...
    ///
    /// [D3DDEVTYPE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddevtype
    pub const fn from_unchecked(devtype: D3DDEVTYPE) -> Self { Self(devtype) }

    /// Convert a [DevType] into a raw [D3DDEVTYPE].
    ///
    /// [D3DDEVTYPE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddevtype
    pub const fn into(self) -> D3DDEVTYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl DevType {
    /// Hardware rasterization. Shading is done with software, hardware, or mixed transform and lighting.
    pub const HAL       : DevType = DevType(D3DDEVTYPE_HAL);

    /// Initialize Direct3D on a computer that has neither hardware nor reference rasterization available, and enable resources for 3D content creation.
    pub const NullRef   : DevType = DevType(D3DDEVTYPE_NULLREF);

    /// Direct3D features are implemented in software; however, the reference rasterizer does make use of special CPU instructions whenever it can.
    ///
    /// The reference device is installed by the Windows SDK 8.0 or later and is intended as an aid in debugging for development only.
    pub const Ref       : DevType = DevType(D3DDEVTYPE_REF);

    /// A pluggable software device that has been registered with [IDirect3D9::RegisterSoftwareDevice](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-registersoftwaredevice)
    pub const SW        : DevType = DevType(D3DDEVTYPE_SW);
}

#[doc(hidden)]
impl DevType {
    /// Initialize Direct3D on a computer that has neither hardware nor reference rasterization available, and enable resources for 3D content creation.
    pub const NULLREF   : DevType = DevType(D3DDEVTYPE_NULLREF);

    /// Direct3D features are implemented in software; however, the reference rasterizer does make use of special CPU instructions whenever it can.
    ///
    /// The reference device is installed by the Windows SDK 8.0 or later and is intended as an aid in debugging for development only.
    pub const REF       : DevType = DevType(D3DDEVTYPE_REF);
}

impl Debug for DevType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DevType::HAL        => write!(f, "DevType::HAL"),
            DevType::NullRef    => write!(f, "DevType::NullRef"),
            DevType::Ref        => write!(f, "DevType::Ref"),
            DevType::SW         => write!(f, "DevType::SW"),
            other               => write!(f, "DevType({})", other.0 as u32),
        }
    }
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default
impl Default for DevType {
    fn default() -> Self { DevType::HAL }
}

impl From<DevType> for D3DDEVTYPE {
    fn from(value: DevType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DDEVTYPE> for DevType {
    fn from(value: D3DDEVTYPE) -> Self { Self(value) }
}
