use winapi::shared::d3d9::{IDirect3DPixelShader9, IDirect3DVertexShader9};
use winapi::um::unknwnbase::IUnknown;

use crate::*;
use crate::d3d9::*;

use std::convert::*;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dpixelshader9)\]
/// A [pixel/fragment shader](https://en.wikipedia.org/wiki/Shader#Pixel_shaders) is a GPU program, run on rasterized fragments.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::create_pixel_shader]
/// *   [IDirect3DDevice9Ext::set_pixel_shader]
/// *   [IDirect3DDevice9Ext::get_pixel_shader]
/// *   [IDirect3DDevice9Ext::set_pixel_shader_constant_b]
/// *   [IDirect3DDevice9Ext::set_pixel_shader_constant_f]
/// *   [IDirect3DDevice9Ext::set_pixel_shader_constant_fv]
/// *   [IDirect3DDevice9Ext::set_pixel_shader_constant_i]
/// *   [IDirect3DDevice9Ext::set_pixel_shader_constant_iv]
#[derive(Clone)] #[repr(transparent)]
pub struct PixelShader(pub(crate) mcom::Rc<IDirect3DPixelShader9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexshader9)\]
/// A [vertex shader](https://en.wikipedia.org/wiki/Shader#Vertex_shaders) transforms mesh verticies when rendering.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::create_vertex_shader]
/// *   [IDirect3DDevice9Ext::set_vertex_shader]
/// *   [IDirect3DDevice9Ext::get_vertex_shader]
/// *   [IDirect3DDevice9Ext::set_vertex_shader_constant_b]
/// *   [IDirect3DDevice9Ext::set_vertex_shader_constant_f]
/// *   [IDirect3DDevice9Ext::set_vertex_shader_constant_fv]
/// *   [IDirect3DDevice9Ext::set_vertex_shader_constant_i]
/// *   [IDirect3DDevice9Ext::set_vertex_shader_constant_iv]
#[derive(Clone)] #[repr(transparent)]
pub struct VertexShader(pub(crate) mcom::Rc<IDirect3DVertexShader9>);



unsafe impl AsSafe<IUnknown> for PixelShader  { fn as_safe(&self) -> &IUnknown { &**self.0 } }
unsafe impl AsSafe<IUnknown> for VertexShader { fn as_safe(&self) -> &IUnknown { &**self.0 } }

unsafe impl AsSafe<IDirect3DPixelShader9 > for PixelShader  { fn as_safe(&self) -> &IDirect3DPixelShader9  { &*self.0 } }
unsafe impl AsSafe<IDirect3DVertexShader9> for VertexShader { fn as_safe(&self) -> &IDirect3DVertexShader9 { &*self.0 } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dpixelshader9)\]
/// IDirect3DPixelShader9 extension methods
///
/// ### Methods
/// | thindx                                                        | docs.microsoft.com    | Description |
/// | ------------------------------------------------------------- | --------------------- | ----------- |
/// | [get_device](Self::get_device)                                | [GetDevice]           | Gets the device.
/// | [get_function_size](Self::get_function_size)                  | [GetFunction]         | Gets the size of the shader function data.
/// | [get_function_inplace](Self::get_function_inplace)            | [GetFunction]         | Gets the shader function data.
/// | [get_function](Self::get_function)                            | [GetFunction]         | Gets the shader function data.
///
/// [GetDevice]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dpixelshader9-getdevice
/// [GetFunction]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dpixelshader9-getfunction
///
pub trait IDirect3DPixelShader9Ext : AsSafe<IDirect3DPixelShader9> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dpixelshader9-getdevice)\]
    /// IDirect3DPixelShader9::GetDevice
    ///
    /// Gets the [Device].
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok([Device])
    fn get_device(&self) -> Result<Device, MethodError> {
        fn_context!(d3d9::IDirect3DPixelShader9Ext::get_device => IDirect3DPixelShader9::GetDevice);
        let mut device = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetDevice(&mut device) })?;
        Ok(unsafe { Device::from_raw(device) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dpixelshader9-getfunction)\]
    /// IDirect3DPixelShader9::GetFunction
    ///
    /// Gets the size of the shader function data.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok(`size`)
    fn get_function_size(&self) -> Result<u32, MethodError> {
        fn_context!(d3d9::IDirect3DPixelShader9Ext::get_function_size => IDirect3DPixelShader9::GetFunction);
        let mut size = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetFunction(null_mut(), &mut size) })?;
        Ok(size)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dpixelshader9-getfunction)\]
    /// IDirect3DPixelShader9::GetFunction
    ///
    /// Gets the shader function data.
    ///
    /// ### Returns
    /// *   [THINERR::SLICE_TOO_LARGE]  `data` exceeded 4GB
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok(`&data[???]`)        Function data was read
    fn get_function_inplace<'d>(&self, data: &'d mut [u8]) -> Result<&'d [u8], MethodError> {
        fn_context!(d3d9::IDirect3DPixelShader9Ext::get_function_inplace => IDirect3DPixelShader9::GetFunction);
        let mut size = data.len().try_into().map_err(|_| fn_param_error!(data, THINERR::SLICE_TOO_LARGE))?;
        // XXX: Do I need a get_function_size check in here too?
        fn_check_hr!(unsafe { self.as_winapi().GetFunction(data.as_mut_ptr().cast(), &mut size) })?;
        Ok(&data[0..(size as usize)])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dpixelshader9-getfunction)\]
    /// IDirect3DPixelShader9::GetFunction
    ///
    /// Gets the shader function data.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok([Vec]&lt;[u8]&gt;)    Function data was read
    fn get_function(&self) -> Result<Vec<u8>, MethodError> {
        fn_context!(d3d9::IDirect3DPixelShader9Ext::get_function => IDirect3DPixelShader9::GetFunction);
        let mut size = self.get_function_size()?;
        let mut data = vec![0u8; size as usize];
        fn_check_hr!(unsafe { self.as_winapi().GetFunction(data.as_mut_ptr().cast(), &mut size) })?;
        debug_assert_eq!(data.len(), size as usize);
        Ok(data)
    }
}

impl<T: AsSafe<IDirect3DPixelShader9>> IDirect3DPixelShader9Ext for T {}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexshader9)\]
/// IDirect3DVertexShader9 extension methods
///
/// ### Methods
/// | thindx                                                        | docs.microsoft.com    | Description |
/// | ------------------------------------------------------------- | --------------------- | ----------- |
/// | [get_device](Self::get_device)                                | [GetDevice]           | Gets the device.
/// | [get_function_size](Self::get_function_size)                  | [GetFunction]         | Gets the size of the shader function data.
/// | [get_function_inplace](Self::get_function_inplace)            | [GetFunction]         | Gets the shader function data.
/// | [get_function](Self::get_function)                            | [GetFunction]         | Gets the shader function data.
///
/// [GetDevice]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexshader9-getdevice
/// [GetFunction]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexshader9-getfunction
///
pub trait IDirect3DVertexShader9Ext : AsSafe<IDirect3DVertexShader9> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexshader9-getdevice)\]
    /// IDirect3DVertexShader9::GetDevice
    ///
    /// Gets the [Device].
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok([Device])
    fn get_device(&self) -> Result<Device, MethodError> {
        fn_context!(d3d9::IDirect3DVertexShader9Ext::get_device => IDirect3DVertexShader9::GetDevice);
        let mut device = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetDevice(&mut device) })?;
        Ok(unsafe { Device::from_raw(device) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexshader9-getfunction)\]
    /// IDirect3DVertexShader9::GetFunction
    ///
    /// Gets the size of the shader function data.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok(`size`)
    fn get_function_size(&self) -> Result<u32, MethodError> {
        fn_context!(d3d9::IDirect3DVertexShader9Ext::get_function_size => IDirect3DVertexShader9::GetFunction);
        let mut size = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetFunction(null_mut(), &mut size) })?;
        Ok(size)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexshader9-getfunction)\]
    /// IDirect3DVertexShader9::GetFunction
    ///
    /// Gets the shader function data.
    ///
    /// ### Returns
    /// *   [THINERR::SLICE_TOO_LARGE]  `data` exceeded 4GB
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok(`&data[???]`)        Function data was read
    fn get_function_inplace<'d>(&self, data: &'d mut [u8]) -> Result<&'d [u8], MethodError> {
        fn_context!(d3d9::IDirect3DVertexShader9Ext::get_function_inplace => IDirect3DVertexShader9::GetFunction);
        let mut size = data.len().try_into().map_err(|_| fn_param_error!(data, THINERR::SLICE_TOO_LARGE))?;
        // XXX: Do I need a get_function_size check in here too?
        fn_check_hr!(unsafe { self.as_winapi().GetFunction(data.as_mut_ptr().cast(), &mut size) })?;
        Ok(&data[0..(size as usize)])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexshader9-getfunction)\]
    /// IDirect3DVertexShader9::GetFunction
    ///
    /// Gets the shader function data.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   The device was pure?
    /// *   Ok([Vec]&lt;[u8]&gt;)    Function data was read
    fn get_function(&self) -> Result<Vec<u8>, MethodError> {
        fn_context!(d3d9::IDirect3DVertexShader9Ext::get_function => IDirect3DVertexShader9::GetFunction);
        let mut size = self.get_function_size()?;
        let mut data = vec![0u8; size as usize];
        fn_check_hr!(unsafe { self.as_winapi().GetFunction(data.as_mut_ptr().cast(), &mut size) })?;
        debug_assert_eq!(data.len(), size as usize);
        Ok(data)
    }
}

impl<T: AsSafe<IDirect3DVertexShader9>> IDirect3DVertexShader9Ext for T {}



// TODO: testing, glorious testing
// TODO: examples



//#cpp2rust IDirect3DPixelShader9                   = d3d9::PixelShader
//#cpp2rust IDirect3DPixelShader9                   = d3d9::IDirect3DPixelShader9Ext

//#cpp2rust IDirect3DVertexShader9                  = d3d9::VertexShader
//#cpp2rust IDirect3DVertexShader9                  = d3d9::IDirect3DVertexShader9Ext
