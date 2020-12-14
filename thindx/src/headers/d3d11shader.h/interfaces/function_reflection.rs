use crate::*;
use crate::ctypes::*;
use crate::d3d11::*;

use winapi::um::d3d11shader::*;

use std::marker::PhantomData;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionreflection)\]
/// ID3D11FunctionReflection
///
/// A function-reflection interface accesses function info.
///
/// ### See Also
/// *   [d3d11::LibraryReflection::get_function_by_index]
/// *   [d3d11::LibraryReflection::functions]
#[derive(Clone)] #[repr(transparent)]
pub struct FunctionReflection<'r> {
    ptr:        NonNull<ID3D11FunctionReflection>,
    phantom:    PhantomData<&'r LibraryReflection>,
}

impl<'r> FunctionReflection<'r> {
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, ptr: *mut ID3D11FunctionReflection) -> Self {
        Self {
            ptr:        NonNull::new(ptr).expect("FunctionReflection should never be null"),
            phantom:    PhantomData,
        }
    }

    /// Allow access as a raw winapi pointer type.
    pub fn as_raw(&self) -> *mut ID3D11FunctionReflection {
        self.ptr.as_ptr()
    }
}

impl<'r> FunctionReflection<'r> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getconstantbufferbyindex)\]
    /// ID3D11FunctionReflection::GetConstantBufferByIndex
    ///
    /// Gets a constant buffer by index for a function.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let d3dc = Compiler::new(47).unwrap();
    /// let shader = d3dc.compile_from_file(
    ///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap();
    ///
    /// let r : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    ///
    /// let scale4 = r.functions().unwrap().find(|f|
    ///     f.get_desc().unwrap().name.to_bytes() == b"scale4"
    /// ).unwrap();
    ///
    /// let valid = scale4.get_constant_buffer_by_index(0);
    /// let desc = valid.get_desc().unwrap();
    /// println!("{:#?}", desc);
    ///
    /// let invalid = scale4.get_constant_buffer_by_index(1);
    /// assert_eq!(Some(E::FAIL), invalid.get_desc().err().map(|e| e.kind()));
    /// ```
    ///
    /// ### Output
    /// ```text
    /// ShaderBufferDesc {
    ///     name: Some(
    ///         "ExampleCBuffer",
    ///     ),
    ///     type: CT::CBuffer,
    ///     variables: 1,
    ///     size: 16,
    ///     flags: CBF::None,
    /// }
    /// ```
    #[xallow(missing_argument_docs)]
    pub fn get_constant_buffer_by_index(&self, buffer_index: u32) -> ShaderReflectionConstantBuffer<'r> {
        let ptr = unsafe { self.ptr.as_ref().GetConstantBufferByIndex(buffer_index) };
        unsafe { ShaderReflectionConstantBuffer::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getconstantbufferbyname)\]
    /// ID3D11FunctionReflection::GetConstantBufferByName
    ///
    /// Gets a constant buffer by name for a function.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let d3dc = Compiler::new(47).unwrap();
    /// let shader = d3dc.compile_from_file(
    ///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap();
    ///
    /// let r : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    ///
    /// let scale4 = r.functions().unwrap().find(|f|
    ///     f.get_desc().unwrap().name.to_bytes() == b"scale4"
    /// ).unwrap();
    ///
    /// let valid = scale4.get_constant_buffer_by_name("ExampleCBuffer");
    /// let desc = valid.get_desc().unwrap();
    /// println!("{:#?}", desc);
    ///
    /// let invalid = scale4.get_constant_buffer_by_name("Nonexistant");
    /// assert_eq!(Some(E::FAIL), invalid.get_desc().err().map(|e| e.kind()));
    /// ```
    ///
    /// ### Output
    /// ```text
    /// ShaderBufferDesc {
    ///     name: Some(
    ///         "ExampleCBuffer",
    ///     ),
    ///     type: CT::CBuffer,
    ///     variables: 1,
    ///     size: 16,
    ///     flags: CBF::None,
    /// }
    /// ```
    #[xallow(missing_argument_docs)]
    pub fn get_constant_buffer_by_name(&self, name: impl TryIntoAsCStr) -> ShaderReflectionConstantBuffer<'r> {
        let name = name.try_into().ok();
        let name = name.as_ref().map_or(cstr!("").as_cstr(), |n| n.as_cstr());
        let ptr = unsafe { self.ptr.as_ref().GetConstantBufferByName(name) };
        unsafe { ShaderReflectionConstantBuffer::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getdesc)\]
    /// ID3D11FunctionReflection::GetDesc
    ///
    /// Fills the function descriptor structure for the function.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let d3dc = Compiler::new(47).unwrap();
    /// let shader = d3dc.compile_from_file(
    ///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap();
    ///
    /// let r : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    ///
    /// for f in r.functions().unwrap() {
    ///     let desc = f.get_desc().unwrap();
    ///     println!("{:#?}", desc);
    /// }
    /// ```
    ///
    /// ### Output
    /// ```text
    /// FunctionDesc {
    ///     version: 4293918800,
    ///     creator: Some(
    ///         "Microsoft (R) HLSL Shader Compiler 10.1",
    ///     ),
    ///     flags: Compile::{Debug|NoPreshader},
    ///     constant_buffers: 0,
    ///     bound_resources: 0,
    ///     instruction_count: 3,
    ///     ...,
    ///     name: Some(
    ///         "xyz1",
    ///     ),
    ///     function_parameter_count: 1,
    ///     has_return: true,
    ///     has_10_level_9_vertex_shader: false,
    ///     has_10_level_9_pixel_shader: false,
    /// }
    /// ```
    pub fn get_desc(&self) -> Result<FunctionDesc<'r>, Error> {
        let mut desc = FunctionDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetDesc(desc.as_mut_ptr()) };
        Error::check("ID3D11FunctionReflection::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getfunctionparameter)\]
    /// ID3D11FunctionReflection::GetFunctionParameter
    ///
    /// Gets the function parameter reflector.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    #[xallow(missing_argument_docs)]
    pub fn get_function_parameter(&self, parameter_index: i32) -> FunctionParameterReflection<'r> {
        let ptr = unsafe { self.ptr.as_ref().GetFunctionParameter(parameter_index) };
        unsafe { FunctionParameterReflection::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getresourcebindingdesc)\]
    /// ID3D11FunctionReflection::GetResourceBindingDesc
    ///
    /// Gets a description of how a resource is bound to a function.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    #[xallow(missing_argument_docs)]
    pub fn get_resource_binding_desc(&self, resource_index: u32) -> Result<ShaderInputBindDesc<'r>, Error> {
        let mut desc = ShaderInputBindDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetResourceBindingDesc(resource_index, desc.as_mut_ptr()) };
        Error::check("ID3D11FunctionReflection::GetResourceBindingDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getresourcebindingdescbyname)\]
    /// ID3D11FunctionReflection::GetResourceBindingDescByName
    ///
    /// Gets a description of how a resource is bound to a function.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    #[xallow(missing_argument_docs)]
    pub fn get_resource_binding_desc_by_name(&self, name: impl TryIntoAsCStr) -> Result<ShaderInputBindDesc<'r>, Error> {
        let name = name.try_into().map_err(|e| Error::new("ID3D11FunctionReflection::GetResourceBindingDescByName", e))?;
        let mut desc = ShaderInputBindDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetResourceBindingDescByName(name.as_cstr(), desc.as_mut_ptr()) };
        Error::check("ID3D11FunctionReflection::GetResourceBindingDescByName", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getvariablebyname)\]
    /// ID3D11FunctionReflection::GetVariableByName
    ///
    /// Gets a variable by name.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    #[xallow(missing_argument_docs)]
    pub fn get_variable_by_name(&self, name: impl TryIntoAsCStr) -> ShaderReflectionVariable<'r> {
        let name = name.try_into().ok();
        let name = name.as_ref().map_or(cstr!("").as_cstr(), |n| n.as_cstr());
        let ptr = unsafe { self.ptr.as_ref().GetVariableByName(name) };
        unsafe { ShaderReflectionVariable::from_raw(self.phantom, ptr) }
    }
}
