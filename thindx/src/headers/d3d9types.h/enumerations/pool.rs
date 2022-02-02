#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpool)\]
/// D3DPOOL
///
/// Defines the memory class that holds the buffers for a resource.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Pool(D3DPOOL);

enumish! { Pool => D3DPOOL; Default, Managed, SystemMem, Scratch }

#[allow(non_upper_case_globals)] impl Pool { // These are enum-like
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
    pub const Default   : Pool = Pool(D3DPOOL_DEFAULT); // 0

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

impl Default for Pool {
    fn default() -> Self { Pool::Default } // 0
}

//#cpp2rust D3DPOOL             = d3d::Pool
//#cpp2rust D3DPOOL_DEFAULT     = d3d::Pool::Default
//#cpp2rust D3DPOOL_MANAGED     = d3d::Pool::Managed
//#cpp2rust D3DPOOL_SYSTEMMEM   = d3d::Pool::SystemMem
//#cpp2rust D3DPOOL_SCRATCH     = d3d::Pool::Scratch
