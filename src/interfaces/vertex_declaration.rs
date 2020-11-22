#![allow(dead_code)] // TODO: remove

use crate::*;

use winapi::shared::d3d9types::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexdeclaration9)\]
/// Describes the layout of the contents of a [VertexBuffer]
#[derive(Clone)] #[repr(transparent)]
pub struct VertexDeclaration(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DVertexDeclaration9>);



/// # VertexDeclarations
/// Bind/Create [VertexDeclaration]s for describing [VertexBuffer] layouts
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexdeclaration)\]
    /// IDirect3DDevice9::CreateVertexDeclaration
    ///
    /// Create a vertex shader declaration from the device and the vertex elements.
    ///
    /// See the [Vertex Declaration (Direct3D 9)] page for a detailed description of how to map vertex declarations between different versions of DirectX.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if `elements.last()` != `Some(D3DDECL_END)`
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([VertexDeclaration])
    ///
    /// [Vertex Declaration (Direct3D 9)]:          https://docs.microsoft.com/en-us/windows/desktop/direct3d9/vertex-declaration
    pub(crate) fn create_vertex_declaration(&self, elements: &[D3DVERTEXELEMENT9]) -> Result<VertexDeclaration, MethodError> {
        let end = elements.last().ok_or(MethodError("Device::create_vertex_declaration", D3DERR::INVALIDCALL))?;
        // This check is required for CreateVertexDeclaration to be sound!
        if !ve_eq(&end, &D3DDECL_END) { return Err(MethodError("Device::create_vertex_declaration", D3DERR::INVALIDCALL)); }

        let mut vd = null_mut();
        let hr = unsafe { self.0.CreateVertexDeclaration(elements.as_ptr(), &mut vd) };
        MethodError::check("IDirect3DDevice9::CreateVertexDeclaration", hr)?;
        Ok(unsafe { VertexDeclaration::from_raw(vd) })
    }
}

// #[test] fn create_vertex_declaration() {} // TODO



fn ve_eq(left: &D3DVERTEXELEMENT9, right: &D3DVERTEXELEMENT9) -> bool {
    left.Stream     == right.Stream &&
    left.Offset     == right.Offset &&
    left.Type       == right.Type   &&
    left.Method     == right.Method &&
    left.Usage      == right.Usage  &&
    left.UsageIndex == right.UsageIndex
}
