#[allow(unused_imports)] use crate::*;
use bytemuck::{Pod, Zeroable};
use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3ddevtype)\]
/// D3DDEVTYPE
///
/// All methods of the [Direct3D] interface that take a [DevType] will fail if [DevType::NullRef] is specified. To use these methods, substitute [DevType::Ref] in the method call.
///
/// A [DevType::Ref] device should be created in [Pool::Scratch] memory, unless vertex and index buffers are required. To support vertex and index buffers, create the device in [Pool::SystemMem] memory.
///
/// If D3dref9.dll is installed, Direct3D will use the reference rasterizer to create a [DevType::Ref] device type, even if [DevType::NullRef] is specified. If D3dref9.dll is not available and [DevType::NullRef] is specified, Direct3D will neither render nor present the scene.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct DevType(D3DDEVTYPE);

enumish! { DevType => D3DDEVTYPE; default: HAL != 0; HAL, NullRef, Ref, SW }

#[allow(non_upper_case_globals)] impl DevType { // These are enum-like
    /// Hardware rasterization. Shading is done with software, hardware, or mixed transform and lighting.
    pub const HAL       : DevType = DevType(D3DDEVTYPE_HAL); // 1

    /// Initialize Direct3D on a computer that has neither hardware nor reference rasterization available, and enable resources for 3D content creation.
    pub const NullRef   : DevType = DevType(D3DDEVTYPE_NULLREF);

    /// Direct3D features are implemented in software; however, the reference rasterizer does make use of special CPU instructions whenever it can.
    ///
    /// The reference device is installed by the Windows SDK 8.0 or later and is intended as an aid in debugging for development only.
    pub const Ref       : DevType = DevType(D3DDEVTYPE_REF);

    /// A pluggable software device that has been registered with [IDirect3D9::RegisterSoftwareDevice](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9-registersoftwaredevice)
    pub const SW        : DevType = DevType(D3DDEVTYPE_SW);
}

//#cpp2rust D3DDEVTYPE          = d3d::DevType

//#cpp2rust D3DDEVTYPE_HAL      = d3d::DevType::HAL
//#cpp2rust D3DDEVTYPE_NULLREF  = d3d::DevType::NullRef
//#cpp2rust D3DDEVTYPE_REF      = d3d::DevType::Ref
//#cpp2rust D3DDEVTYPE_SW       = d3d::DevType::SW
