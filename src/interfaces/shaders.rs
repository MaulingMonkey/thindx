use crate::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dpixelshader9)\]
/// A [pixel/fragment shader](https://en.wikipedia.org/wiki/Shader#Pixel_shaders) is a GPU program, run on rasterized fragments.
#[derive(Clone)] #[repr(transparent)]
pub struct PixelShader(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DPixelShader9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexshader9)\]
/// A [vertex shader](https://en.wikipedia.org/wiki/Shader#Vertex_shaders) transforms mesh verticies when rendering.
#[derive(Clone)] #[repr(transparent)]
pub struct VertexShader(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DVertexShader9>);



/// # Shaders
/// Bind/Create [PixelShader]s and [VertexShader]s
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createpixelshader)\]
    /// IDirect3DDevice9::CreatePixelShader
    ///
    /// Creates a pixel shader.
    ///
    /// ### Safety
    ///
    /// The caller must pass a valid shader blob.
    /// The underlying Direct3D API is unsound - it doesn't even take a length for the DWORD array.
    /// This function will likely attempt to validate the shader blob bytecode in the future and/or offload such validation onto the parameter, but until then this is unsound as heck.
    /// Do not trust user-generated-content for shader bytecode blobs.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok([PixelShader])
    pub unsafe fn create_pixel_shader(&self, function: &[u32]) -> Result<PixelShader, MethodError> {
        let mut shader = null_mut();
        let hr = self.0.CreatePixelShader(function.as_ptr(), &mut shader);
        MethodError::check("IDirect3DDevice9::CreatePixelShader", hr)?;
        Ok(PixelShader::from_raw(shader))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexshader)\]
    /// IDirect3DDevice9::CreateVertexShader
    ///
    /// Creates a vertex shader.
    ///
    /// ### Safety
    ///
    /// The caller must pass a valid shader blob.
    /// The underlying Direct3D API is unsound - it doesn't even take a length for the DWORD array.
    /// This function will likely attempt to validate the shader blob bytecode in the future and/or offload such validation onto the parameter, but until then this is unsound as heck.
    /// Do not trust user-generated-content for shader bytecode blobs.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok([VertexShader])
    pub unsafe fn create_vertex_shader(&self, function: &[u32]) -> Result<VertexShader, MethodError> {
        let mut shader = null_mut();
        let hr = self.0.CreateVertexShader(function.as_ptr(), &mut shader);
        MethodError::check("IDirect3DDevice9::CreateVertexShader", hr)?;
        Ok(VertexShader::from_raw(shader))
    }
}

// #[test] fn create_pixel_shader() {} // TODO
// #[test] fn create_vertex_shader() {} // TODO
