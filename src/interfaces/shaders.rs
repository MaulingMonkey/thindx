use crate::*;

use std::convert::*;
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

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshader)\]
    /// IDirect3DDevice9::SetPixelShader
    ///
    /// Sets the pixel shader to render with.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If `pixel_shader` was created by another device?
    /// *   Ok(())
    pub fn set_pixel_shader<'sh>(&self, pixel_shader: impl Into<Option<&'sh PixelShader>>) -> Result<(), MethodError> {
        let pixel_shader = pixel_shader.into();
        let ps = pixel_shader.map_or(null_mut(), |ps| ps.as_raw());
        let hr = unsafe { self.0.SetPixelShader(ps) };
        MethodError::check("IDirect3DDevice9::SetPixelShader", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshader)\]
    /// IDirect3DDevice9::SetVertexShader
    ///
    /// Sets the vertex shader to render with.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If `vertex_shader` was created by another device?
    /// *   Ok(())
    pub fn set_vertex_shader<'sh>(&self, vertex_shader: impl Into<Option<&'sh VertexShader>>) -> Result<(), MethodError> {
        let vertex_shader = vertex_shader.into();
        let ps = vertex_shader.map_or(null_mut(), |ps| ps.as_raw());
        let hr = unsafe { self.0.SetVertexShader(ps) };
        MethodError::check("IDirect3DDevice9::SetVertexShader", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getpixelshader)\]
    /// IDirect3DDevice9::GetPixelShader
    ///
    /// Gets the pixel shader currently bound to the device, if any.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the device was created with D3DCREATE_PUREDEVICE?
    /// *   Ok(Some([PixelShader]))     If a pixel shader was bound
    /// *   Ok(None)                    If no pixel shader was bound
    pub fn get_pixel_shader(&self) -> Result<PixelShader, MethodError> {
        let mut shader = null_mut();
        let hr = unsafe { self.0.GetPixelShader(&mut shader) };
        MethodError::check("IDirect3DDevice9::GetPixelShader", hr)?;
        Ok(unsafe { PixelShader::from_raw(shader) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getvertexshader)\]
    /// IDirect3DDevice9::GetVertexShader
    ///
    /// Gets the vertex shader currently bound to the device, if any.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the device was created with D3DCREATE_PUREDEVICE?
    /// *   Ok(Some([VertexShader]))    If a vertex shader was bound
    /// *   Ok(None)                    If no vertex shader was bound
    pub fn get_vertex_shader(&self) -> Result<VertexShader, MethodError> {
        let mut shader = null_mut();
        let hr = unsafe { self.0.GetVertexShader(&mut shader) };
        MethodError::check("IDirect3DDevice9::GetVertexShader", hr)?;
        Ok(unsafe { VertexShader::from_raw(shader) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstantb)\]
    /// IDirect3DDevice9::SetPixelShaderConstantB
    ///
    /// Sets boolean shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max boolean registers?
    /// *   Ok(())
    pub fn set_pixel_shader_constant_b(&self, start_register: u32, constant_data: &[bool32]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_b", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetPixelShaderConstantB(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantB", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstantf)\]
    /// IDirect3DDevice9::SetPixelShaderConstantF
    ///
    /// Sets floating-point shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() / 4` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() % 4` != `0` (not a multiple of vectors)
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()/4` > max floating point vector registers?
    /// *   Ok(())
    pub fn set_pixel_shader_constant_f(&self, start_register: u32, constant_data: &[f32]) -> Result<(), MethodError> {
        let n = constant_data.len();
        if (n % 4) != 0 { return Err(MethodError("Device::set_pixel_shader_constant_f", D3DERR::INVALIDCALL)); }
        let n : u32 = (n/4).try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_f", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetPixelShaderConstantF(start_register, constant_data.as_ptr(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstantf)\]
    /// IDirect3DDevice9::SetPixelShaderConstantF
    ///
    /// Sets floating-point shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers?
    /// *   Ok(())
    pub fn set_pixel_shader_constant_fv(&self, start_register: u32, constant_data: &[[f32; 4]]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_fv", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetPixelShaderConstantF(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstanti)\]
    /// IDirect3DDevice9::SetPixelShaderConstantI
    ///
    /// Sets integer shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() / 4` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() % 4` != `0` (not a multiple of vectors)
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()/4` > max floating point vector registers?
    /// *   Ok(())
    pub fn set_pixel_shader_constant_i(&self, start_register: u32, constant_data: &[i32]) -> Result<(), MethodError> {
        let n = constant_data.len();
        if (n % 4) != 0 { return Err(MethodError("Device::set_pixel_shader_constant_i", D3DERR::INVALIDCALL)); }
        let n : u32 = (n/4).try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_i", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetPixelShaderConstantI(start_register, constant_data.as_ptr(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantI", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstanti)\]
    /// IDirect3DDevice9::SetPixelShaderConstantI
    ///
    /// Sets integer shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers?
    /// *   Ok(())
    pub fn set_pixel_shader_constant_iv(&self, start_register: u32, constant_data: &[[i32; 4]]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_iv", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetPixelShaderConstantI(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantI", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstantb)\]
    /// IDirect3DDevice9::SetVertexShaderConstantB
    ///
    /// Sets boolean shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max boolean registers?
    /// *   Ok(())
    pub fn set_vertex_shader_constant_b(&self, start_register: u32, constant_data: &[bool32]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_b", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetVertexShaderConstantB(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantB", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstantf)\]
    /// IDirect3DDevice9::SetVertexShaderConstantF
    ///
    /// Sets floating-point shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() / 4` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() % 4` != `0` (not a multiple of vectors)
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()/4` > max floating point vector registers?
    /// *   Ok(())
    pub fn set_vertex_shader_constant_f(&self, start_register: u32, constant_data: &[f32]) -> Result<(), MethodError> {
        let n = constant_data.len();
        if (n % 4) != 0 { return Err(MethodError("Device::set_vertex_shader_constant_f", D3DERR::INVALIDCALL)); }
        let n : u32 = (n/4).try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_f", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetVertexShaderConstantF(start_register, constant_data.as_ptr(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstantf)\]
    /// IDirect3DDevice9::SetVertexShaderConstantF
    ///
    /// Sets floating-point shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers?
    /// *   Ok(())
    pub fn set_vertex_shader_constant_fv(&self, start_register: u32, constant_data: &[[f32; 4]]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_fv", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetVertexShaderConstantF(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstanti)\]
    /// IDirect3DDevice9::SetVertexShaderConstantI
    ///
    /// Sets integer shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() / 4` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() % 4` != `0` (not a multiple of vectors)
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()/4` > max floating point vector registers?
    /// *   Ok(())
    pub fn set_vertex_shader_constant_i(&self, start_register: u32, constant_data: &[i32]) -> Result<(), MethodError> {
        let n = constant_data.len();
        if (n % 4) != 0 { return Err(MethodError("Device::set_vertex_shader_constant_i", D3DERR::INVALIDCALL)); }
        let n : u32 = (n/4).try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_i", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetVertexShaderConstantI(start_register, constant_data.as_ptr(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantI", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstanti)\]
    /// IDirect3DDevice9::SetVertexShaderConstantI
    ///
    /// Sets integer shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers?
    /// *   Ok(())
    pub fn set_vertex_shader_constant_iv(&self, start_register: u32, constant_data: &[[i32; 4]]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_iv", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.0.SetVertexShaderConstantI(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantI", hr)
    }
}

// #[test] fn create_pixel_shader() {} // TODO
// #[test] fn create_vertex_shader() {} // TODO
