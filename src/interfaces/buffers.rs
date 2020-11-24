#![allow(dead_code)] // TODO: remove

use crate::*;

use winapi::ctypes::c_void;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dindexbuffer9)\]
/// (extends [Resource])
/// An [index buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing)
/// indexes verticies in a [VertexBuffer] when rendering.
///
/// ### See Also
///
/// *   [Device::create_index_buffer]
/// *   [Device::set_indices]
/// *   [Device::get_indices]
#[derive(Clone)] #[repr(transparent)]
pub struct IndexBuffer(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DIndexBuffer9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexbuffer9)\]
/// (extends [Resource])
/// A [vertex buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing)
/// typically contains points of a mesh to be rendered.
///
/// ### See Also
///
/// *   [Device::create_vertex_buffer]
/// *   [Device::set_stream_source]
/// *   [Device::set_stream_source_freq]
/// *   [Device::get_stream_source]
/// *   [Device::get_stream_source_freq]
#[derive(Clone)] #[repr(transparent)]
pub struct VertexBuffer(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DVertexBuffer9>);



/// # Buffers
/// Bind/Create/Update [IndexBuffer]s and [VertexBuffer]s
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createindexbuffer)\]
    /// IDirect3DDevice9::CreateIndexBuffer
    ///
    /// Creates an index buffer.
    ///
    /// ### Parameters
    ///
    /// *   `length`            Size of the index buffer, **in bytes**.
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `format`            Typically [Format::Index16] or [Format::Index32] (type of index buffer)
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://docs.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let single_triangle = device.create_index_buffer(3 * 2, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one index (2 for [Format::Index16], 4 for [Format::Index32])
    /// *   [D3DERR::INVALIDCALL]       if `usage`, `format`, or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::INVALIDDATA]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok([IndexBuffer])
    pub fn create_index_buffer(&self, length: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, shared_handle: impl SharedHandleParam) -> Result<IndexBuffer, MethodError> {
        let _ = shared_handle;
        let mut buffer = null_mut();
        let hr = unsafe { self.0.CreateIndexBuffer(length, usage.into().into(), format.into().into(), pool.into().into(), &mut buffer, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateIndexBuffer", hr)?;
        Ok(unsafe { IndexBuffer::from_raw(buffer) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexbuffer)\]
    /// IDirect3DDevice9::CreateVertexBuffer
    ///
    /// Creates an vertex buffer.
    ///
    /// ### Parameters
    ///
    /// *   `length`            Size of the index buffer, **in bytes**.
    ///                         For FVF vertex buffers, Length must be large enough to contain at least one vertex, but it need not be a multiple of the vertex size.
    ///                         Length is not validated for non-FVF buffers.
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `fvf`               Combination of [FVF], a usage specifier that describes the vertex format of the verticies in this buffer.
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://docs.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let vert_size = 3 * 4; // XYZ * floats
    /// let length = 3 * vert_size; // 3 verts
    /// let single_triangle = device.create_vertex_buffer(length, Usage::None, FVF::XYZ, Pool::Managed, ()).unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one [FVF]-sized vertex (1 if [FVF::None])
    /// *   [D3DERR::INVALIDCALL]       if `usage` or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok([VertexBuffer])
    pub fn create_vertex_buffer(&self, length: u32, usage: impl Into<Usage>, fvf: impl Into<FVF>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<VertexBuffer, MethodError> {
        let mut buffer = null_mut();
        let hr = unsafe { self.0.CreateVertexBuffer(length, usage.into().into(), fvf.into().into(), pool.into().into(), &mut buffer, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateVertexBuffer", hr)?;
        Ok(unsafe { VertexBuffer::from_raw(buffer) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setindices)\]
    /// IDirect3DDevice9::SetIndices
    ///
    /// Sets index data.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let tri = device.create_index_buffer(3*2, Usage::None, Format::Index16, Pool::Default, ()).unwrap();
    /// // ...initialize tri...
    ///
    /// device.set_indices(&tri).unwrap();          // bind the index buffer
    /// device.set_indices(Some(&tri)).unwrap();    // bind the index buffer
    /// device.set_indices(None).unwrap();          // unbind the index buffer
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       (perhaps only on an invalid [IndexBuffer] that thin3d9's API prevents? perhaps when mixing IBs from different devices?)
    /// *   Ok(())
    pub fn set_indices<'ib>(&self, index_data: impl Into<Option<&'ib IndexBuffer>>) -> Result<(), MethodError> {
        let index_data = index_data.into();
        let ptr = index_data.map_or(null_mut(), |id| id.as_raw());
        let hr = unsafe { self.0.SetIndices(ptr) };
        MethodError::check("IDirect3DDevice9::SetIndices", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getindices)\]
    /// IDirect3DDevice9::GetIndices
    ///
    /// Retrieves index data.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let ib : Option<IndexBuffer> = device.get_indices().unwrap();
    /// assert!(ib.is_none(), "device has no index buffer by default");
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if the device is a pure device?
    /// *   Ok(Some([IndexBuffer]))     if an index buffer was bound
    /// *   Ok(None)                    if no index buffer was bound
    pub fn get_indices(&self) -> Result<Option<IndexBuffer>, MethodError> {
        let mut buffer = null_mut();
        let hr = unsafe { self.0.GetIndices(&mut buffer) };
        MethodError::check("IDirect3DDevice9::GetIndices", hr)?;
        Ok(unsafe { IndexBuffer::from_raw_opt(buffer) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsource)\]
    /// IDirect3DDevice9::SetStreamSource
    ///
    /// Binds a vertex buffer to a device data stream. For more information, see [Setting the Stream Source (Direct3D 9)].
    ///
    /// [Setting the Stream Source (Direct3D 9)]:       https://docs.microsoft.com/en-us/windows/desktop/direct3d9/setting-the-stream-source
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if the [VertexBuffer] belongs to another device?
    /// *   Ok(`()`)
    pub fn set_stream_source<'b>(&self, stream_number: u32, stream_data: impl Into<Option<&'b VertexBuffer>>, offset_in_bytes: u32, stride: u32) -> Result<(), MethodError> {
        let stream_data = stream_data.into();
        let stream_data = stream_data.map_or(null_mut(), |sd| sd.as_raw());
        let hr = unsafe { self.0.SetStreamSource(stream_number, stream_data, offset_in_bytes, stride) };
        MethodError::check("IDirect3DDevice9::SetStreamSource", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsourcefreq)\]
    /// IDirect3DDevice9::SetStreamSourceFreq
    ///
    /// Sets the stream source frequency divider value. This may be used to draw several instances of geometry.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    pub fn set_stream_source_freq(&self, stream_number: u32, setting: impl Into<StreamSource>) -> Result<(), MethodError> {
        let setting = setting.into().into();
        let hr = unsafe { self.0.SetStreamSourceFreq(stream_number, setting) };
        MethodError::check("IDirect3DDevice9::SetStreamSourceFreq", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getstreamsource)\]
    /// IDirect3DDevice9::GetStreamSource
    ///
    /// Retrieves a vertex buffer bound to the specified data stream.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   The device was a pure device?
    /// *   Ok(Some([VertexBuffer]), `offset_in_bytes`, `stride`)
    /// *   Ok(None, 0, 0)
    pub fn get_stream_source(&self, stream_number: u32) -> Result<(Option<VertexBuffer>, u32, u32), MethodError> {
        let mut buffer = null_mut();
        let mut offset = 0;
        let mut stride = 0;
        let hr = unsafe { self.0.GetStreamSource(stream_number, &mut buffer, &mut offset, &mut stride) };
        MethodError::check("IDirect3DDevice9::GetStreamSource", hr)?;
        let buffer = unsafe { VertexBuffer::from_raw_opt(buffer) };
        Ok((buffer, offset, stride))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getstreamsourcefreq)\]
    /// IDirect3DDevice9::GetStreamSourceFreq
    ///
    /// Gets the stream source frequency divider value.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   The device was a pure device?
    /// *   Ok([StreamSource])
    pub fn get_stream_source_freq(&self, stream_number: u32) -> Result<StreamSource, MethodError> {
        let mut freq = 0;
        let hr = unsafe { self.0.GetStreamSourceFreq(stream_number, &mut freq) };
        MethodError::check("IDirect3DDevice9::GetStreamSourceFreq", hr)?;
        Ok(StreamSource::from_unchecked(freq))
    }
}



impl IndexBuffer {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dindexbuffer9-getdesc)\]
    /// IDirect3DIndexBuffer9::GetDesc
    ///
    /// Retrieves a description of the index buffer.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok([IndexBufferDesc])
    pub fn get_desc(&self) -> Result<IndexBufferDesc, MethodError> {
        let mut desc = IndexBufferDesc::default();
        let hr = unsafe { self.0.GetDesc(&mut *desc) };
        MethodError::check("IDirect3DIndexBuffer9::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dindexbuffer9-lock)\]
    /// IDirect3DIndexBuffer9::Lock
    ///
    /// ### Safety
    ///
    /// *   Invalid, out-of-bounds `offset` / `size`s might be unsound
    /// *   Violating the constraints imposed by `flags` is definitely unsound
    /// *   Having multiple writers to the resulting pointer (including from other locks?) is also super unsound
    ///
    /// Sound users of this API will lock, modify, and unlock in such a way as to prevent any other concurrent modifier of the data in question.
    /// This is simplified by the Direct3D APIs being \![Send], \![Sync], but care must be involved with traits that could execute arbitrary code.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(data)
    pub unsafe fn lock_unchecked(&self, offset: u32, size: u32, flags: impl Into<Lock>) -> Result<*mut c_void, MethodError> {
        let mut data = null_mut();
        let hr = self.0.Lock(offset, size, &mut data, flags.into().into());
        MethodError::check("IDirect3DIndexBuffer9::Lock", hr)?;
        Ok(data)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dindexbuffer9-unlock)\]
    /// IDirect3DIndexBuffer9::Unlock
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   The index buffer wasn't locked?
    /// *   Ok(`()`)
    pub fn unlock(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.0.Unlock() };
        MethodError::check("IDirect3DIndexBuffer9::Unlock", hr)
    }
}



impl VertexBuffer {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexbuffer9-getdesc)\]
    /// IDirect3DVertexBuffer9::GetDesc
    ///
    /// Retrieves a description of the vertex buffer.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok([VertexBufferDesc])
    pub fn get_desc(&self) -> Result<VertexBufferDesc, MethodError> {
        let mut desc = VertexBufferDesc::default();
        let hr = unsafe { self.0.GetDesc(&mut *desc) };
        MethodError::check("IDirect3DVertexBuffer9::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexbuffer9-lock)\]
    /// IDirect3DVertexBuffer9::Lock
    ///
    /// ### Safety
    ///
    /// *   Invalid, out-of-bounds `offset` / `size`s might be unsound
    /// *   Violating the constraints imposed by `flags` is definitely unsound
    /// *   Having multiple writers to the resulting pointer (including from other locks?) is also super unsound
    ///
    /// Sound users of this API will lock, modify, and unlock in such a way as to prevent any other concurrent modifier of the data in question.
    /// This is simplified by the Direct3D APIs being \![Send], \![Sync], but care must be involved with traits that could execute arbitrary code.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(data)
    pub unsafe fn lock_unchecked(&self, offset: u32, size: u32, flags: impl Into<Lock>) -> Result<*mut c_void, MethodError> {
        let mut data = null_mut();
        let hr = self.0.Lock(offset, size, &mut data, flags.into().into());
        MethodError::check("IDirect3DVertexBuffer9::Lock", hr)?;
        Ok(data)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexbuffer9-unlock)\]
    /// IDirect3DVertexBuffer9::Unlock
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   The vertex buffer wasn't locked?
    /// *   Ok(`()`)
    pub fn unlock(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.0.Unlock() };
        MethodError::check("IDirect3DVertexBuffer9::Unlock", hr)
    }
}



#[test] fn index_buffer() {
    let device = Device::test();

    let tri16 = device.create_index_buffer(3*2, Usage::None, Format::Index16, Pool::Default, ()).unwrap(); // simple triangle IB
    let odd16 = device.create_index_buffer(3*3, Usage::None, Format::Index16, Pool::Default, ()).unwrap(); // weird size (9)
    let tri32 = device.create_index_buffer(3*4, Usage::None, Format::Index32, Pool::Default, ()).unwrap(); // simple triangle IB

    assert_eq!(D3DERR::INVALIDCALL, device.create_index_buffer(0,    Usage::None, Format::Index16,  Pool::Default, ()).err(), "empty");
    assert_eq!(D3DERR::INVALIDCALL, device.create_index_buffer(1,    Usage::None, Format::Index16,  Pool::Default, ()).err(), "too small for 16-bit IndexBuffer");
    assert_eq!(D3DERR::INVALIDCALL, device.create_index_buffer(3,    Usage::None, Format::Index32,  Pool::Default, ()).err(), "too small for 32-bit IndexBuffer");
    assert_eq!(D3DERR::INVALIDCALL, device.create_index_buffer(1000, Invalid,     Format::Index16,  Pool::Default, ()).err(), "invalid usage");
    assert_eq!(D3DERR::INVALIDCALL, device.create_index_buffer(1000, Usage::None, Format::Unknown,  Pool::Default, ()).err(), "bad format");
    assert_eq!(D3DERR::INVALIDCALL, device.create_index_buffer(1000, Usage::None, Format::X8B8G8R8, Pool::Default, ()).err(), "bad format");
    assert_eq!(D3DERR::INVALIDCALL, device.create_index_buffer(1000, Usage::None, Invalid,          Pool::Default, ()).err(), "bad format");
    assert_eq!(D3DERR::INVALIDCALL, device.create_index_buffer(1000, Usage::None, Format::Index16,  Invalid,       ()).err(), "bad pool");

    assert!(device.get_indices().unwrap().is_none());

    device.set_indices(&tri16).unwrap();
    assert_eq!(device.get_indices().unwrap().unwrap().as_raw(), tri16.as_raw());
    device.set_indices(Some(&tri16)).unwrap();
    assert_eq!(device.get_indices().unwrap().unwrap().as_raw(), tri16.as_raw());

    device.set_indices(&odd16).unwrap();
    assert_eq!(device.get_indices().unwrap().unwrap().as_raw(), odd16.as_raw());
    device.set_indices(Some(&odd16)).unwrap();
    assert_eq!(device.get_indices().unwrap().unwrap().as_raw(), odd16.as_raw());

    device.set_indices(&tri32).unwrap();
    assert_eq!(device.get_indices().unwrap().unwrap().as_raw(), tri32.as_raw());
    device.set_indices(Some(&tri32)).unwrap();
    assert_eq!(device.get_indices().unwrap().unwrap().as_raw(), tri32.as_raw());

    device.set_indices(None).unwrap();
    assert!(device.get_indices().unwrap().is_none());

    // TODO: multiple device tests
}

#[test] fn create_vertex_buffer() {
    let device = Device::test();

    let _trixyz = device.create_vertex_buffer(3*4*3, Usage::None, FVF::None, Pool::Default, ()).unwrap(); // simple triangle VB (no FVF)
    let _quadxyz= device.create_vertex_buffer(4*4*3, Usage::None, FVF::None, Pool::Default, ()).unwrap(); // simple quad     VB (no FVF)
    let _trixyz = device.create_vertex_buffer(3*4*3, Usage::None, FVF::XYZ,  Pool::Default, ()).unwrap(); // simple triangle VB (FVF)
    let _quadxyz= device.create_vertex_buffer(4*4*3, Usage::None, FVF::XYZ,  Pool::Default, ()).unwrap(); // simple quad     VB (FVF)

    let _one =                      device.create_vertex_buffer(1, Usage::None, FVF::None, Pool::Default, ()).unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.create_vertex_buffer(1, Usage::None, FVF::XYZ,  Pool::Default, ()).err()); // 1 byte is too small for FVF::XYZ

    assert_eq!(D3DERR::INVALIDCALL, device.create_vertex_buffer(0,    Usage::None, FVF::XYZ, Pool::Default, ()).err(), "empty");
    assert_eq!(D3DERR::INVALIDCALL, device.create_vertex_buffer(1000, Invalid,     FVF::XYZ, Pool::Default, ()).err(), "invalid usage");
    let _badfmt =                   device.create_vertex_buffer(1000, Usage::None, Invalid,  Pool::Default, ()).unwrap(); // apparently no such thing as a bad FVF?
    assert_eq!(D3DERR::INVALIDCALL, device.create_vertex_buffer(1000, Usage::None, FVF::XYZ, Invalid,       ()).err(), "bad pool");

    // TODO: multiple device tests
}

// TODO: a lot more testing
// TODO: examples
