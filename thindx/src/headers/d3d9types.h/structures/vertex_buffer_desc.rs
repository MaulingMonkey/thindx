use crate::d3d9::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/D3DVERTEXBUFFER_DESC)\]
/// D3DVERTEXBUFFER_DESC
///
/// Describes an vertex buffer
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct VertexBufferDesc {
    pub format:     Format,
    pub ty:         ResourceType,
    pub usage:      Usage,
    pub pool:       Pool,
    pub size:       u32,
    pub fvf:        FVF,
}

impl Deref    for VertexBufferDesc { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DVERTEXBUFFER_DESC; }
impl DerefMut for VertexBufferDesc { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DVERTEXBUFFER_DESC> for VertexBufferDesc { fn from(value: D3DVERTEXBUFFER_DESC) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<VertexBufferDesc> for D3DVERTEXBUFFER_DESC { fn from(value: VertexBufferDesc    ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! { VertexBufferDesc => D3DVERTEXBUFFER_DESC { format => Format, ty => Type, usage => Usage, pool => Pool, size => Size, fvf => FVF } }

//#cpp2rust D3DVERTEXBUFFER_DESC = d3d::VertexBufferDesc
