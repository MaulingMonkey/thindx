#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpool)\]
/// D3DPOOL
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Pool(D3DPOOL);

impl Pool {
    /// Convert a raw [D3DPOOL] value into a [Pool].  This is *probably* safe... probably....
    ///
    /// [D3DPOOL]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpool
    pub const fn from_unchecked(pool: D3DPOOL) -> Self { Self(pool) }

    /// Convert a [Pool] into a raw [D3DPOOL].
    ///
    /// [D3DPOOL]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpool
    pub const fn into(self) -> D3DPOOL { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Pool {
    /// Resources are placed in the memory pool most appropriate for the set of usages requested for the given resource.
    /// This is usually video memory, including both local video memory and AGP memory.
    /// The [Pool::Default] pool is separate from [Pool::Managed] and [Pool::SystemMem], and it specifies that the resource is placed in the preferred memory for device access.
    /// Note that [Pool::Default] never indicates that either [Pool::Managed] or [Pool::SystemMem] should be chosen as the memory pool type for this resource.
    /// Textures placed in the [Pool::Default] pool cannot be locked unless they are dynamic textures or they are private, FOURCC, driver formats.
    /// To access unlockable textures, you must use functions such as [IDirect3DDevice9::UpdateSurface], [IDirect3DDevice9::UpdateTexture], [IDirect3DDevice9::GetFrontBufferData], and [IDirect3DDevice9::GetRenderTargetData].
    /// [Pool::Managed] is probably a better choice than [Pool::Default] for most applications.\
    /// Note that some textures created in driver-proprietary pixel formats, unknown to the Direct3D runtime, can be locked.
    /// Also note that - unlike textures - swap chain back buffers, render targets, vertex buffers, and index buffers can be locked.
    /// When a device is lost, resources created using [Pool::Default] must be released before calling [IDirect3DDevice9::Reset].
    /// For more information, see [Lost Devices (Direct3D 9)].
    ///
    /// [IDirect3DDevice9::UpdateSurface]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-updatesurface
    /// [IDirect3DDevice9::UpdateTexture]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-updatetexture
    /// [IDirect3DDevice9::GetFrontBufferData]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getfrontbufferdata
    /// [IDirect3DDevice9::GetRenderTargetData]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getrendertargetdata
    /// [IDirect3DDevice9::Reset]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-reset
    /// [Lost Devices (Direct3D 9)]:                https://docs.microsoft.com/en-us/windows/win32/direct3d9/lost-devices
    pub const Default   : Pool = Pool(D3DPOOL_DEFAULT);

    /// Resources are copied automatically to device-accessible memory as needed.
    /// Managed resources are backed by system memory and do not need to be recreated when a device is lost.
    /// See [Managing Resources (Direct3D 9)] for more information.
    /// Managed resources can be locked. Only the system-memory copy is directly modified.
    /// Direct3D copies your changes to driver-accessible memory as needed.
    ///
    /// [Managing Resources (Direct3D 9)]:          https://docs.microsoft.com/en-us/windows/win32/direct3d9/managing-resources
    pub const Managed   : Pool = Pool(D3DPOOL_MANAGED);

    /// Resources are placed in memory that is not typically accessible by the Direct3D device.
    /// This memory allocation consumes system RAM but does not reduce pageable RAM.
    /// These resources do not need to be recreated when a device is lost.
    /// Resources in this pool can be locked and can be used as the source for a IDirect3DDevice9::UpdateSurface or IDirect3DDevice9::UpdateTexture operation to a memory resource created with [Pool::Default].
    ///
    /// [IDirect3DDevice9::UpdateSurface]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-updatesurface
    /// [IDirect3DDevice9::UpdateTexture]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-updatetexture
    pub const SystemMem : Pool = Pool(D3DPOOL_SYSTEMMEM);

    /// Resources are placed in system RAM and do not need to be recreated when a device is lost.
    /// These resources are not bound by device size or format restrictions.
    /// Because of this, these resources cannot be accessed by the Direct3D device nor set as textures or render targets.
    /// However, these resources can always be created, locked, and copied.
    pub const Scratch   : Pool = Pool(D3DPOOL_SCRATCH);
}

#[doc(hidden)] impl Pool {
    /// Resources are placed in the memory pool most appropriate for the set of usages requested for the given resource.
    /// This is usually video memory, including both local video memory and AGP memory.
    /// The [Pool::Default] pool is separate from [Pool::Managed] and [Pool::SystemMem], and it specifies that the resource is placed in the preferred memory for device access.
    /// Note that [Pool::Default] never indicates that either [Pool::Managed] or [Pool::SystemMem] should be chosen as the memory pool type for this resource.
    /// Textures placed in the [Pool::Default] pool cannot be locked unless they are dynamic textures or they are private, FOURCC, driver formats.
    /// To access unlockable textures, you must use functions such as [IDirect3DDevice9::UpdateSurface], [IDirect3DDevice9::UpdateTexture], [IDirect3DDevice9::GetFrontBufferData], and [IDirect3DDevice9::GetRenderTargetData].
    /// [Pool::Managed] is probably a better choice than [Pool::Default] for most applications.\
    /// Note that some textures created in driver-proprietary pixel formats, unknown to the Direct3D runtime, can be locked.
    /// Also note that - unlike textures - swap chain back buffers, render targets, vertex buffers, and index buffers can be locked.
    /// When a device is lost, resources created using [Pool::Default] must be released before calling [IDirect3DDevice9::Reset].
    /// For more information, see [Lost Devices (Direct3D 9)].
    ///
    /// [IDirect3DDevice9::UpdateSurface]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-updatesurface
    /// [IDirect3DDevice9::UpdateTexture]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-updatetexture
    /// [IDirect3DDevice9::GetFrontBufferData]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getfrontbufferdata
    /// [IDirect3DDevice9::GetRenderTargetData]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getrendertargetdata
    /// [IDirect3DDevice9::Reset]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-reset
    /// [Lost Devices (Direct3D 9)]:                https://docs.microsoft.com/en-us/windows/win32/direct3d9/lost-devices
    pub const DEFAULT   : Pool = Pool(D3DPOOL_DEFAULT);

    /// Resources are copied automatically to device-accessible memory as needed.
    /// Managed resources are backed by system memory and do not need to be recreated when a device is lost.
    /// See [Managing Resources (Direct3D 9)] for more information.
    /// Managed resources can be locked. Only the system-memory copy is directly modified.
    /// Direct3D copies your changes to driver-accessible memory as needed.
    ///
    /// [Managing Resources (Direct3D 9)]:          https://docs.microsoft.com/en-us/windows/win32/direct3d9/managing-resources
    pub const MANAGED   : Pool = Pool(D3DPOOL_MANAGED);

    /// Resources are placed in memory that is not typically accessible by the Direct3D device.
    /// This memory allocation consumes system RAM but does not reduce pageable RAM.
    /// These resources do not need to be recreated when a device is lost.
    /// Resources in this pool can be locked and can be used as the source for a IDirect3DDevice9::UpdateSurface or IDirect3DDevice9::UpdateTexture operation to a memory resource created with [Pool::Default].
    ///
    /// [IDirect3DDevice9::UpdateSurface]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-updatesurface
    /// [IDirect3DDevice9::UpdateTexture]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-updatetexture
    pub const SYSTEMMEM : Pool = Pool(D3DPOOL_SYSTEMMEM);

    /// Resources are placed in system RAM and do not need to be recreated when a device is lost.
    /// These resources are not bound by device size or format restrictions.
    /// Because of this, these resources cannot be accessed by the Direct3D device nor set as textures or render targets.
    /// However, these resources can always be created, locked, and copied.
    pub const SCRATCH   : Pool = Pool(D3DPOOL_SCRATCH);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for Pool {
    fn default() -> Self { Pool::Default }
}

impl Debug for Pool {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Pool::Default   => write!(f, "Pool::Default"),
            Pool::Managed   => write!(f, "Pool::Managed"),
            Pool::SystemMem => write!(f, "Pool::SystemMem"),
            Pool::Scratch   => write!(f, "Pool::Scratch"),
            other           => write!(f, "Pool({})", other.0),
        }
    }
}

impl From<Pool> for D3DPOOL {
    fn from(value: Pool) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DPOOL> for Pool {
    fn from(value: D3DPOOL) -> Self { Self(value) }
}
