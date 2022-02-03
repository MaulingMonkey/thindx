use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dindexbuffer-desc)\]
/// D3DINDEXBUFFER_DESC
///
/// Describes an index buffer
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct IndexBufferDesc {
    pub format:     Format,
    pub ty:         ResourceType,
    pub usage:      Usage,
    pub pool:       Pool,
    pub size:       u32,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    IndexBufferDesc => D3DINDEXBUFFER_DESC {
        format  => Format,
        ty      => Type,
        usage   => Usage,
        pool    => Pool,
        size    => Size
    }
}

//#cpp2rust D3DINDEXBUFFER_DESC = d3d::IndexBufferDesc
