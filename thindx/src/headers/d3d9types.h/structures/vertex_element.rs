use crate::d3d9::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvertexelement9)\]
/// D3DVERTEXELEMENT9
///
/// Defines the vertex data layout. Each vertex can contain one or more data types, and each data type is described by a vertex element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] pub struct VertexElement {
    pub stream:         u16,
    pub offset:         u16,
    pub ty:             DeclType8,
    pub method:         DeclMethod8,
    pub usage:          DeclUsage8,
    pub usage_index:    u8,
}

impl VertexElement {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddecl-end)\]
    /// D3DDECL_END
    pub const END : VertexElement = VertexElement {
        stream:         0xFF,
        offset:         0,
        ty:             DeclType8::Unused,      // 17
        method:         DeclMethod8::Default,   // 0
        usage:          DeclUsage8::Position,   // 0
        usage_index:    0,
    };
}

impl Deref    for VertexElement { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DVERTEXELEMENT9; }
impl DerefMut for VertexElement { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DVERTEXELEMENT9> for VertexElement { fn from(value: D3DVERTEXELEMENT9) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<VertexElement> for D3DVERTEXELEMENT9 { fn from(value: VertexElement    ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! { VertexElement => D3DVERTEXELEMENT9 { stream => Stream, offset => Offset, ty => Type, method => Method, usage_index => UsageIndex } }
