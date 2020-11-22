#![allow(dead_code)] // TODO: remove

use crate::*;

type FVF = u32;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dindexbuffer9)\]
/// (extends [Resource])
/// An [index buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing)
/// indexes verticies in a [VertexBuffer] when rendering.
#[derive(Clone)] #[repr(transparent)]
pub struct IndexBuffer(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DIndexBuffer9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexbuffer9)\]
/// (extends [Resource])
/// A [vertex buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing)
/// typically contains points of a mesh to be rendered.
#[derive(Clone)] #[repr(transparent)]
pub struct VertexBuffer(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DVertexBuffer9>);



/// # Buffers
/// Bind/Create/Update [IndexBuffer]s and [VertexBuffer]s
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createindexbuffer)\]
    /// IDirect3DDevice9::CreateIndexBuffer
    ///
    /// Creates an index buffer.
    pub(crate) fn create_index_buffer(&self, length: u32, usage: Usage, format: Format, pool: Pool, _shared_handle: impl SharedHandleParam) -> Result<IndexBuffer, MethodError> {
        let mut buffer = null_mut();
        let hr = unsafe { self.0.CreateIndexBuffer(length, usage.into(), format.into(), pool.into(), &mut buffer, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateIndexBuffer", hr)?;
        Ok(unsafe { IndexBuffer::from_raw(buffer) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexbuffer)\]
    /// IDirect3DDevice9::CreateVertexBuffer
    ///
    /// Creates an vertex buffer.
    pub(crate) fn create_vertex_buffer(&self, length: u32, usage: Usage, fvf: FVF, pool: Pool, _shared_handle: impl SharedHandleParam) -> Result<VertexBuffer, MethodError> {
        let mut buffer = null_mut();
        let hr = unsafe { self.0.CreateVertexBuffer(length, usage.into(), fvf, pool.into(), &mut buffer, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateVertexBuffer", hr)?;
        Ok(unsafe { VertexBuffer::from_raw(buffer) })
    }
}

// #[test] fn create_index_buffer() {} // TODO
// #[test] fn create_vertex_buffer() {} // TODO
