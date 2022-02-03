use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/D3DVERTEXBUFFER_DESC)\]
/// D3DVERTEXBUFFER_DESC
///
/// Describes an vertex buffer
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct VertexBufferDesc {
    pub format:     Format,
    pub ty:         ResourceType,
    pub usage:      Usage,
    pub pool:       Pool,
    pub size:       u32,
    pub fvf:        FVF,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    VertexBufferDesc => D3DVERTEXBUFFER_DESC {
        format  => Format,
        ty      => Type,
        usage   => Usage,
        pool    => Pool,
        size    => Size,
        fvf     => FVF,
    }
}

//#cpp2rust D3DVERTEXBUFFER_DESC = d3d::VertexBufferDesc
