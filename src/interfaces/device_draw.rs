use crate::*;

use winapi::ctypes::c_void;



/// # Drawing
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitive)\]
    /// IDirect3DDevice9::DrawIndexedPrimitive
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    pub unsafe fn draw_indexed_primitive(&self, primitive_type: PrimitiveType, base_vertex_index: i32, min_vertex_index: u32, num_verticies: u32, start_index: u32, primitive_count: u32) -> Result<(), MethodError> {
        let hr = self.0.DrawIndexedPrimitive(primitive_type.into(), base_vertex_index, min_vertex_index, num_verticies, start_index, primitive_count);
        MethodError::check("IDirect3DDevice9::DrawIndexedPrimitive", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitive)\]
    /// IDirect3DDevice9::DrawIndexedPrimitive
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    pub unsafe fn draw_indexed_primitive_up(&self, primitive_type: PrimitiveType, min_vertex_index: u32, num_verticies: u32, primitive_count: u32, index_data: impl IndexData, vertex_stream_zero: impl VertexStreamData) -> Result<(), MethodError> {
        let hr = self.0.DrawIndexedPrimitiveUP(primitive_type.into(), min_vertex_index, num_verticies, primitive_count, index_data.ptr(), index_data.format().into(), vertex_stream_zero.ptr(), vertex_stream_zero.stride());
        MethodError::check("IDirect3DDevice9::DrawIndexedPrimitiveUP", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitive)\]
    /// IDirect3DDevice9::DrawPrimitive
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    pub unsafe fn draw_primitive(&self, primitive_type: PrimitiveType, start_vertex: u32, primitive_count: u32) -> Result<(), MethodError> {
        let hr = self.0.DrawPrimitive(primitive_type.into(), start_vertex, primitive_count);
        MethodError::check("IDirect3DDevice9::DrawPrimitive", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitiveup)\]
    /// IDirect3DDevice9::DrawPrimitiveUP
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    pub unsafe fn draw_primitive_up(&self, primitive_type: PrimitiveType, primitive_count: u32, vertex_stream_zero: impl VertexStreamData) -> Result<(), MethodError> {
        let hr = self.0.DrawPrimitiveUP(primitive_type.into(), primitive_count, vertex_stream_zero.ptr(), vertex_stream_zero.stride());
        MethodError::check("IDirect3DDevice9::DrawPrimitiveUP", hr)
    }

    // TODO: DrawRectPatch
    // TODO: DrawTriPatch

    // TODO: docs for args, remarks, deep links, etc.
    // TODO: safety sections
}

// #[test] fn draw_indexed_primitive() {}
// #[test] fn draw_indexed_primitive_up() {}
// #[test] fn draw_primitive() {}
// #[test] fn draw_primitive_up() {}
// #[test] fn draw_rect_patch() {}
// #[test] fn draw_tri_patch() {}



/// [Device::draw_indexed_primitive_up] index data
pub unsafe trait IndexData {
    fn count(&self) -> usize;
    fn ptr(&self) -> *const c_void;
    fn format(&self) -> Format;
}

unsafe impl IndexData for &[u16] {
    fn count(&self) -> usize { self.len() }
    fn ptr(&self) -> *const c_void { self.as_ptr().cast() }
    fn format(&self) -> Format { Format::INDEX16 }
}

unsafe impl IndexData for &[u32] {
    fn count(&self) -> usize { self.len() }
    fn ptr(&self) -> *const c_void { self.as_ptr().cast() }
    fn format(&self) -> Format { Format::INDEX32 }
}



pub unsafe trait VertexStreamData {
    fn count(&self) -> usize;
    fn ptr(&self) -> *const c_void;
    fn stride(&self) -> u32;
}

unsafe impl<E: Copy> VertexStreamData for &[E] {
    fn count(&self) -> usize { self.len() }
    fn ptr(&self) -> *const c_void { self.as_ptr().cast() }
    fn stride(&self) -> u32 { std::mem::size_of::<E>() as u32 }
}
