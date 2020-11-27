#![allow(dead_code)] // TODO: remove

use crate::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexdeclaration9)\]
/// Describes the layout of the contents of a [VertexBuffer]
#[derive(Clone)] #[repr(transparent)]
pub struct VertexDeclaration(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DVertexDeclaration9>);



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
    pub fn create_vertex_declaration(&self, elements: &[VertexElement]) -> Result<VertexDeclaration, MethodError> {
        let end = elements.last().ok_or(MethodError("Device::create_vertex_declaration", D3DERR::INVALIDCALL))?;
        // This check is required for CreateVertexDeclaration to be sound!
        if *end != VertexElement::END { return Err(MethodError("Device::create_vertex_declaration", D3DERR::INVALIDCALL)); }

        let mut vd = null_mut();
        let hr = unsafe { self.0.CreateVertexDeclaration(elements.as_ptr().cast(), &mut vd) };
        MethodError::check("IDirect3DDevice9::CreateVertexDeclaration", hr)?;
        Ok(unsafe { VertexDeclaration::from_raw(vd) })
    }
}



impl VertexDeclaration {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdeclaration)\]
    /// IDirect3DVertexDeclaration9::GetDeclaration
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the device is a pure device?
    /// *   Ok([u32])               The number of elements in this vertex declaration, including the [VertexElement::END]
    pub fn get_declaration_size(&self) -> Result<u32, MethodError> {
        let mut num_elements = 0;
        let hr = unsafe { self.0.GetDeclaration(null_mut(), &mut num_elements) };
        MethodError::check("IDirect3DVertexDeclaration9::GetDeclaration", hr)?;
        Ok(num_elements)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdeclaration)\]
    /// IDirect3DVertexDeclaration9::GetDeclaration
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the device is a pure device?
    /// *   [D3DERR::INVALIDCALL]   If `elements` is too small to contain the result
    /// *   Ok(&[[VertexElement]])                  If `elements` was successfully written to, including the [VertexElement::END]
    pub fn get_declaration_inplace<'e>(&self, elements: &'e mut [VertexElement]) -> Result<&'e [VertexElement], MethodError> {
        let mut num_elements = self.get_declaration_size()?;
        if num_elements as usize > elements.len() { return Err(MethodError("VertexDeclaration::get_declaration_inplace", D3DERR::INVALIDCALL)); }
        let hr = unsafe { self.0.GetDeclaration(null_mut(), &mut num_elements) };
        MethodError::check("IDirect3DVertexDeclaration9::GetDeclaration", hr)?;
        Ok(&elements[0..(num_elements as usize)])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdeclaration)\]
    /// IDirect3DVertexDeclaration9::GetDeclaration
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]               If the device is a pure device?
    /// *   Ok(Vec&lt;[VertexElement]&gt;)      The elements of this vertex declaration, including the [VertexElement::END]
    pub fn get_declaration(&self) -> Result<Vec<VertexElement>, MethodError> {
        let mut num_elements = self.get_declaration_size()?;
        let mut v = vec![VertexElement::default(); num_elements as usize];
        let hr = unsafe { self.0.GetDeclaration(v.as_mut_ptr().cast(), &mut num_elements) };
        debug_assert!(v.len() == num_elements as usize); // size didn't change, right?
        MethodError::check("IDirect3DVertexDeclaration9::GetDeclaration", hr)?;
        Ok(v)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdevice)\]
    /// IDirect3DVertexDeclaration9::GetDevice
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device is a pure device?
    /// *   Ok([Device])
    pub fn get_device(&self) -> Result<Device, MethodError> {
        let mut device = null_mut();
        let hr = unsafe { self.0.GetDevice(&mut device) };
        MethodError::check("IDirect3DVertexDeclaration9::GetDevice", hr)?;
        Ok(unsafe { Device::from_raw(device) })
    }
}

// #[test] fn create_vertex_declaration() {} // TODO

// TODO: test coverage, examples
