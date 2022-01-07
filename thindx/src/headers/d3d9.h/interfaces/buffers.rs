use crate::*;
use crate::d3d9::*;

use winapi::ctypes::c_void;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dindexbuffer9)\]
/// (extends [Resource])
/// An [index buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing)
/// indexes verticies in a [VertexBuffer] when rendering.
///
/// ### See Also
///
/// *   [IDirect3DDevice9Ext::create_index_buffer]
/// *   [IDirect3DDevice9Ext::set_indices]
/// *   [IDirect3DDevice9Ext::get_indices]
#[derive(Clone)] #[repr(transparent)]
pub struct IndexBuffer(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DIndexBuffer9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexbuffer9)\]
/// (extends [Resource])
/// A [vertex buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing)
/// typically contains points of a mesh to be rendered.
///
/// ### See Also
///
/// *   [IDirect3DDevice9Ext::create_vertex_buffer]
/// *   [IDirect3DDevice9Ext::set_stream_source]
/// *   [IDirect3DDevice9Ext::set_stream_source_freq]
/// *   [IDirect3DDevice9Ext::get_stream_source]
/// *   [IDirect3DDevice9Ext::get_stream_source_freq]
#[derive(Clone)] #[repr(transparent)]
pub struct VertexBuffer(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DVertexBuffer9>);



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dindexbuffer9)\]
/// IDirect3DIndexBuffer9 extension methods
///
/// ### Methods
///
/// | thin3d9                                   | docs.microsoft.com    | Description |
/// | ----------------------------------------- | --------------------- | ----------- |
/// | [get_desc](Self::get_desc)                | [GetDesc]             | Retrieves a description of the index buffer.
/// | [lock_unchecked](Self::lock_unchecked)    | [Lock]                | Locks a range of index data, and obtains a pointer to the index buffer memory.
/// | [unlock](Self::unlock)                    | [Unlock]              | Unlocks index data.
///
/// [GetDesc]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dindexbuffer9-getdesc
/// [Lock]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dindexbuffer9-lock
/// [Unlock]:   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dindexbuffer9-unlock
///
pub trait IDirect3DIndexBuffer9Ext : index_buffer::Sealed {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dindexbuffer9-getdesc)\]
    /// IDirect3DIndexBuffer9::GetDesc
    ///
    /// Retrieves a description of the index buffer.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_index_buffer(3 * 2, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// let desc : IndexBufferDesc = tri.get_desc().unwrap();
    /// assert_eq!(desc.format, Format::Index16);
    /// assert_eq!(desc.ty,     ResourceType::IndexBuffer);
    /// assert_eq!(desc.usage,  Usage::None);
    /// assert_eq!(desc.pool,   Pool::Managed);
    /// assert_eq!(desc.size,   6);
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([IndexBufferDesc])
    fn get_desc(&self) -> Result<IndexBufferDesc, MethodError> {
        let mut desc = IndexBufferDesc::default();
        let hr = unsafe { self.as_winapi().GetDesc(&mut *desc) };
        MethodError::check("IDirect3DIndexBuffer9::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dindexbuffer9-lock)\]
    /// IDirect3DIndexBuffer9::Lock
    ///
    /// Locks a range of index data, and obtains a pointer to the index buffer memory.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// *   Invalid, out-of-bounds `offset` / `size`s might be unsound
    /// *   Violating the constraints imposed by `flags` is definitely unsound
    /// *   Having multiple writers to the resulting pointer (including from other locks?) is also super unsound
    ///
    /// Sound users of this API will lock, modify, and unlock in such a way as to prevent any other concurrent modifier of the data in question.
    /// This is simplified by the Direct3D APIs being \![Send], \![Sync], but care must be involved with traits that could execute arbitrary code.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_index_buffer(3 * 2, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// let data : &[u16] = &[0, 1, 2];
    /// unsafe {
    ///     let d = tri.lock_unchecked(0, 0, Lock::None).unwrap();
    ///     std::ptr::copy_nonoverlapping(data.as_ptr(), d.cast(), data.len());
    /// }
    /// tri.unlock().unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(data)
    unsafe fn lock_unchecked(&self, offset: u32, size: u32, flags: impl Into<Lock>) -> Result<*mut c_void, MethodError> {
        let mut data = null_mut();
        let hr = self.as_winapi().Lock(offset, size, &mut data, flags.into().into());
        MethodError::check("IDirect3DIndexBuffer9::Lock", hr)?;
        Ok(data)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dindexbuffer9-unlock)\]
    /// IDirect3DIndexBuffer9::Unlock
    ///
    /// Unlocks index data.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_index_buffer(3*4*3, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// tri.unlock().unwrap(); // may succeed, even if the buffer wasn't locked <_<;;
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   The index buffer wasn't locked?
    /// *   Ok(`()`)
    fn unlock(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().Unlock() };
        MethodError::check("IDirect3DIndexBuffer9::Unlock", hr)
    }
}

impl<T: index_buffer::Sealed> IDirect3DIndexBuffer9Ext for T {}

mod index_buffer {
    use winapi::shared::d3d9::IDirect3DIndexBuffer9;
    pub unsafe trait Sealed                                 { fn as_winapi(&self) -> &IDirect3DIndexBuffer9; }
    unsafe impl Sealed for mcom::Rc<IDirect3DIndexBuffer9>  { fn as_winapi(&self) -> &IDirect3DIndexBuffer9 { &**self } }
    unsafe impl Sealed for super::IndexBuffer               { fn as_winapi(&self) -> &IDirect3DIndexBuffer9 { &*self.0 } }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexbuffer9)\]
/// IDirect3DVertexBuffer9 extension methods
///
/// ### Methods
///
/// | thin3d9                                   | docs.microsoft.com    | Description |
/// | ----------------------------------------- | --------------------- | ----------- |
/// | [get_desc](Self::get_desc)                | [GetDesc]             | Retrieves a description of the vertex buffer.
/// | [lock_unchecked](Self::lock_unchecked)    | [Lock]                | Locks a range of vertex data, and obtains a pointer to the vertex buffer memory.
/// | [unlock](Self::unlock)                    | [Unlock]              | Unlocks vertex data.
///
/// [GetDesc]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexbuffer9-getdesc
/// [Lock]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexbuffer9-lock
/// [Unlock]:   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexbuffer9-unlock
///
pub trait IDirect3DVertexBuffer9Ext : vertex_buffer::Sealed {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexbuffer9-getdesc)\]
    /// IDirect3DVertexBuffer9::GetDesc
    ///
    /// Retrieves a description of the vertex buffer.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_vertex_buffer(3*4*3, Usage::None, FVF::XYZ, Pool::Managed, ()).unwrap();
    /// let desc : VertexBufferDesc = dbg!(tri.get_desc().unwrap());
    /// assert_eq!(desc.format, Format::VertexData);
    /// assert_eq!(desc.ty,     ResourceType::VertexBuffer);
    /// assert_eq!(desc.usage,  Usage::None);
    /// assert_eq!(desc.pool,   Pool::Managed);
    /// assert_eq!(desc.size,   36);
    /// assert_eq!(desc.fvf,    FVF::XYZ);
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([VertexBufferDesc])
    fn get_desc(&self) -> Result<VertexBufferDesc, MethodError> {
        let mut desc = VertexBufferDesc::default();
        let hr = unsafe { self.as_winapi().GetDesc(&mut *desc) };
        MethodError::check("IDirect3DVertexBuffer9::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexbuffer9-lock)\]
    /// IDirect3DVertexBuffer9::Lock
    ///
    /// Locks a range of vertex data, and obtains a pointer to the vertex buffer memory.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// *   Invalid, out-of-bounds `offset` / `size`s might be unsound
    /// *   Violating the constraints imposed by `flags` is definitely unsound
    /// *   Having multiple writers to the resulting pointer (including from other locks?) is also super unsound
    ///
    /// Sound users of this API will lock, modify, and unlock in such a way as to prevent any other concurrent modifier of the data in question.
    /// This is simplified by the Direct3D APIs being \![Send], \![Sync], but care must be involved with traits that could execute arbitrary code.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_vertex_buffer(3*4*3, Usage::None, FVF::XYZ, Pool::Managed, ()).unwrap();
    /// let data : &[[f32; 3]] = &[
    ///     [0.0, 1.0, 0.0],
    ///     [1.0, 0.0, 0.0],
    ///     [0.0, 0.0, 1.0],
    /// ];
    /// unsafe {
    ///     let d = tri.lock_unchecked(0, std::mem::size_of_val(data) as u32, Lock::None).unwrap();
    ///     std::ptr::copy_nonoverlapping(data.as_ptr(), d.cast(), data.len());
    /// }
    /// tri.unlock().unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(data)
    unsafe fn lock_unchecked(&self, offset: u32, size: u32, flags: impl Into<Lock>) -> Result<*mut c_void, MethodError> {
        let mut data = null_mut();
        let hr = self.as_winapi().Lock(offset, size, &mut data, flags.into().into());
        MethodError::check("IDirect3DVertexBuffer9::Lock", hr)?;
        Ok(data)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexbuffer9-unlock)\]
    /// IDirect3DVertexBuffer9::Unlock
    ///
    /// Unlocks vertex data.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_vertex_buffer(3*4*3, Usage::None, FVF::XYZ, Pool::Managed, ()).unwrap();
    /// tri.unlock().unwrap(); // may succeed, even if the buffer wasn't locked <_<;;
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    fn unlock(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().Unlock() };
        MethodError::check("IDirect3DVertexBuffer9::Unlock", hr)
    }
}

impl<T: vertex_buffer::Sealed> IDirect3DVertexBuffer9Ext for T {}

mod vertex_buffer {
    use winapi::shared::d3d9::IDirect3DVertexBuffer9;
    pub unsafe trait Sealed                                 { fn as_winapi(&self) -> &IDirect3DVertexBuffer9; }
    unsafe impl Sealed for mcom::Rc<IDirect3DVertexBuffer9> { fn as_winapi(&self) -> &IDirect3DVertexBuffer9 { &**self } }
    unsafe impl Sealed for super::VertexBuffer              { fn as_winapi(&self) -> &IDirect3DVertexBuffer9 { &*self.0 } }
}



#[cfg(test)] mod tests {
    use dev::d3d9::*;

    #[test] fn index_buffer() {
        let device = device_pure();

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
    }

    #[test] fn create_vertex_buffer() {
        let device = device_pure();

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
    }

    #[test] fn overflow_allocs() {
        if testfast() { return; }

        fn index_loop_alloc_size(alloc: u32) -> Result<(), MethodError> {
            let device = device_pure();
            let mut ibs = Vec::new();
            for _n in 0..1000 { ibs.push(device.create_index_buffer(alloc, Usage::None, Format::Index16, Pool::Default, ())?); }
            panic!("expected overflow_allocs::index_loop_alloc_size(0x{:08x}) to fail before collecting 1000 allocs", alloc);
        }

        fn vertex_loop_alloc_size(alloc: u32) -> Result<(), MethodError> {
            let device = device_pure();
            let mut vbs = Vec::new();
            for _n in 0..1000 { vbs.push(device.create_vertex_buffer(alloc, Usage::None, FVF::None, Pool::Default, ())?); }
            panic!("expected overflow_allocs::vertex_loop_alloc_size(0x{:08x}) to fail before collecting 1000 allocs", alloc);
        }

        for n in [
            !0,
            !0-4,
            !0-100,
            super::MAX_BUFFER_ALLOC,
            0xF000_0000,
            !0/2,
            !0/8,
        ].iter().copied() {
            match index_loop_alloc_size(n).unwrap_err().kind() {
                E::OUTOFMEMORY              => {}, // expected
                D3DERR::OUTOFVIDEOMEMORY    => {}, // expected
                THIN3D9ERR::ALLOC_OVERFLOW  => {}, // expected
                other => panic!("index_loop_alloc_size(0x{:08x}), expected a different kind of error, got: {}", n, other),
            }

            match vertex_loop_alloc_size(n).unwrap_err().kind() {
                E::OUTOFMEMORY              => {}, // expected
                D3DERR::OUTOFVIDEOMEMORY    => {}, // expected
                THIN3D9ERR::ALLOC_OVERFLOW  => {}, // expected
                other => panic!("vertex_loop_alloc_size(0x{:08x}), expected a different kind of error, got: {}", n, other),
            }
        }
    }
}
