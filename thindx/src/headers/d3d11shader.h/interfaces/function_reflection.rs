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
/// ### Example
/// ```rust
/// # use thindx::*;
/// let d3dc = d3d::Compiler::load_system(47).unwrap();
/// let shader = d3dc.compile_from_file(
///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
///     d3d::Compile::Debug, d3d::CompileEffect::None
/// ).unwrap();
/// let lib : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
///
/// let scale4 : d3d11::FunctionReflection = lib.functions().unwrap().find(|f|
///     f.get_desc().unwrap().name.to_bytes() == b"scale4"
/// ).unwrap();
/// ```
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
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let lib : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    /// # let scale4 = lib.functions().unwrap().find(|f| f.get_desc().unwrap().name.to_bytes() == b"scale4").unwrap();
    /// #
    /// let valid = scale4.get_constant_buffer_by_index(0);
    /// println!("{:#?}", valid.get_desc().unwrap());
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
    //#allow_missing_argument_docs
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
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let lib : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    /// # let scale4 = lib.functions().unwrap().find(|f| f.get_desc().unwrap().name.to_bytes() == b"scale4").unwrap();
    /// #
    /// let valid = scale4.get_constant_buffer_by_name("ExampleCBuffer");
    /// println!("{:#?}", valid.get_desc().unwrap());
    ///
    /// let invalid = scale4.get_constant_buffer_by_name("Nonexistant");
    /// assert_eq!(Some(E::FAIL), invalid.get_desc().err().map(|e| e.kind()));
    ///
    /// let invalid = scale4.get_constant_buffer_by_name("Non\0existant");
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
    //#allow_missing_argument_docs
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
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let lib : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    /// # let scale4 = lib.functions().unwrap().find(|f| f.get_desc().unwrap().name.to_bytes() == b"scale4").unwrap();
    /// #
    /// println!("{:#?}", scale4.get_desc().unwrap());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// FunctionDesc {
    ///     version: 4293918800,
    ///     creator: "Microsoft (R) HLSL Shader Compiler 10.1",
    ///     flags: Compile::{Debug|NoPreshader},
    ///     constant_buffers: 1,
    ///     bound_resources: 1,
    ///     instruction_count: 2,
    ///     ...,
    ///     name: "scale4",
    ///     function_parameter_count: 1,
    ///     has_return: true,
    ///     has_10_level_9_vertex_shader: false,
    ///     has_10_level_9_pixel_shader: false,
    /// }
    /// ```
    pub fn get_desc(&self) -> Result<FunctionDesc<'r>, MethodError> {
        let mut desc = FunctionDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetDesc(desc.as_mut_ptr()) };
        MethodError::check("ID3D11FunctionReflection::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getfunctionparameter)\]
    /// ID3D11FunctionReflection::GetFunctionParameter
    ///
    /// Gets the function parameter reflector.
    ///
    /// ### Arguments
    /// *   `parameter_index`   - A 0-based parameter index less than `self.get_desc().unwrap().function_parameter_count`, or `-1` to get the return value "parameter".
    ///
    /// ### Returns
    /// *   A stub object that returns [E::FAIL] from [`FunctionParameterReflection::get_desc`] if `parameter_index` is invalid
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let lib : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    /// # let scale4 = lib.functions().unwrap().find(|f| f.get_desc().unwrap().name.to_bytes() == b"scale4").unwrap();
    /// #
    /// let ret = scale4.get_function_parameter(-1).get_desc().unwrap();
    /// assert_eq!(ret.name.as_ref().unwrap().to_bytes(),   b"scale4");
    /// assert_eq!(ret.ty,      d3d::SVT::Float);
    /// assert_eq!(ret.class,   d3d::SVC::Vector);
    /// assert_eq!(ret.rows,    1);
    /// assert_eq!(ret.columns, 4);
    /// println!("{:#?}", ret);
    ///
    /// let v = scale4.get_function_parameter( 0).get_desc().unwrap();
    /// assert_eq!(v.name.as_ref().unwrap().to_bytes(),   b"v");
    /// assert_eq!(v.ty,        d3d::SVT::Float);
    /// assert_eq!(v.class,     d3d::SVC::Vector);
    /// assert_eq!(v.rows,      1);
    /// assert_eq!(v.columns,   4);
    /// println!("{:#?}", v);
    ///
    /// assert_eq!(E::FAIL, scale4.get_function_parameter(-2).get_desc().unwrap_err().kind());
    /// assert_eq!(E::FAIL, scale4.get_function_parameter( 1).get_desc().unwrap_err().kind());
    /// #
    /// # assert_eq!(scale4.get_function_parameter(std::i32::MIN).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(-1000000).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(-10000).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(-100).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(100).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(10000).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(1000000).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(std::i32::MAX-100).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(std::i32::MAX-16).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(std::i32::MAX-10).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(std::i32::MAX-4).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(std::i32::MAX-1).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// # assert_eq!(scale4.get_function_parameter(std::i32::MAX).get_desc().err().map(|err| err.kind()), Some(E::FAIL));
    /// ```
    ///
    /// ### Output
    /// ```text
    /// ParameterDesc {
    ///     name: Some(
    ///         "scale4",
    ///     ),
    ///     semantic_name: None,
    ///     ty: SVT::Float,
    ///     class: SVC::Vector,
    ///     rows: 1,
    ///     columns: 4,
    ///     interpolation_mode: Interpolation::Undefined,
    ///     flags: PF::Out,
    ///     first_in_register: 4294967295,
    ///     first_in_component: 4294967295,
    ///     first_out_register: 0,
    ///     first_out_component: 0,
    /// }
    /// ParameterDesc {
    ///     name: Some(
    ///         "v",
    ///     ),
    ///     semantic_name: None,
    ///     ty: SVT::Float,
    ///     class: SVC::Vector,
    ///     rows: 1,
    ///     columns: 4,
    ///     interpolation_mode: Interpolation::Undefined,
    ///     flags: PF::In,
    ///     first_in_register: 0,
    ///     first_in_component: 0,
    ///     first_out_register: 4294967295,
    ///     first_out_component: 4294967295,
    /// }
    /// ```
    pub fn get_function_parameter(&self, parameter_index: i32) -> FunctionParameterReflection<'r> {
        let ptr = unsafe { self.ptr.as_ref().GetFunctionParameter(parameter_index) };
        unsafe { FunctionParameterReflection::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getresourcebindingdesc)\]
    /// ID3D11FunctionReflection::GetResourceBindingDesc
    ///
    /// Gets a description of how a resource is bound to a function.
    ///
    /// ### Errors
    /// *   [E::INVALIDARG]     - If `resource_index` >= `self.get_desc().unwrap().bound_resources`
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let lib : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    /// # let scale4 = lib.functions().unwrap().find(|f| f.get_desc().unwrap().name.to_bytes() == b"scale4").unwrap();
    /// #
    /// for i in 0..scale4.get_desc().unwrap().bound_resources {
    ///     println!("{:#?}", scale4.get_resource_binding_desc(i).unwrap());
    /// }
    ///
    /// // out of bounds
    /// assert_eq!(
    ///     scale4.get_resource_binding_desc(100).err().map(|err| err.kind()),
    ///     Some(E::INVALIDARG)
    /// );
    /// #
    /// # assert_eq!(scale4.get_resource_binding_desc(10000).err().map(|err| err.kind()), Some(E::INVALIDARG));
    /// # assert_eq!(scale4.get_resource_binding_desc(1000000).err().map(|err| err.kind()), Some(E::INVALIDARG));
    /// # assert_eq!(scale4.get_resource_binding_desc(!0-100).err().map(|err| err.kind()), Some(E::INVALIDARG));
    /// # assert_eq!(scale4.get_resource_binding_desc(!0-16).err().map(|err| err.kind()), Some(E::INVALIDARG));
    /// # assert_eq!(scale4.get_resource_binding_desc(!0-10).err().map(|err| err.kind()), Some(E::INVALIDARG));
    /// # assert_eq!(scale4.get_resource_binding_desc(!0-4).err().map(|err| err.kind()), Some(E::INVALIDARG));
    /// # assert_eq!(scale4.get_resource_binding_desc(!0-1).err().map(|err| err.kind()), Some(E::INVALIDARG));
    /// # assert_eq!(scale4.get_resource_binding_desc(!0).err().map(|err| err.kind()), Some(E::INVALIDARG));
    /// ```
    ///
    /// ### Output
    /// ```text
    /// ShaderInputBindDesc {
    ///     name: "ExampleCBuffer",
    ///     ty: SIT::CBuffer,
    ///     bind_point: 0,
    ///     bind_count: 1,
    ///     flags: SIF::None,
    ///     return_type: ReturnType(0),
    ///     dimension: SrvDimension::Unknown,
    ///     num_samples: 0,
    /// }
    /// ```
    //#allow_missing_argument_docs
    pub fn get_resource_binding_desc(&self, resource_index: u32) -> Result<ShaderInputBindDesc<'r>, MethodError> {
        let mut desc = ShaderInputBindDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetResourceBindingDesc(resource_index, desc.as_mut_ptr()) };
        MethodError::check("ID3D11FunctionReflection::GetResourceBindingDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionreflection-getresourcebindingdescbyname)\]
    /// ID3D11FunctionReflection::GetResourceBindingDescByName
    ///
    /// Gets a description of how a resource is bound to a function.
    ///
    /// ### Errors
    /// *   [E::INVALIDARG]                     - if `name` doesn't name a resource binding
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let lib : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    /// # let scale4 = lib.functions().unwrap().find(|f| f.get_desc().unwrap().name.to_bytes() == b"scale4").unwrap();
    /// #
    /// println!("{:#?}", scale4.get_resource_binding_desc_by_name("ExampleCBuffer").unwrap());
    ///
    /// assert_eq!(
    ///     scale4.get_resource_binding_desc_by_name("Nonexistant").err().map(|err| err.kind()),
    ///     Some(E::INVALIDARG)
    /// );
    ///
    /// assert_eq!(
    ///     scale4.get_resource_binding_desc_by_name("Non\0existant").err().map(|err| err.kind()),
    ///     Some(E::INVALIDARG)
    /// );
    /// ```
    ///
    /// ### Output
    /// ```text
    /// ShaderInputBindDesc {
    ///     name: "ExampleCBuffer",
    ///     ty: SIT::CBuffer,
    ///     bind_point: 0,
    ///     bind_count: 1,
    ///     flags: SIF::None,
    ///     return_type: ReturnType(0),
    ///     dimension: SrvDimension::Unknown,
    ///     num_samples: 0,
    /// }
    /// ```
    //#allow_missing_argument_docs
    pub fn get_resource_binding_desc_by_name(&self, name: impl TryIntoAsCStr) -> Result<ShaderInputBindDesc<'r>, MethodError> {
        let name = name.try_into().ok();
        let name = name.as_ref().map_or(cstr!("").as_cstr(), |n| n.as_cstr());
        let mut desc = ShaderInputBindDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetResourceBindingDescByName(name, desc.as_mut_ptr()) };
        MethodError::check("ID3D11FunctionReflection::GetResourceBindingDescByName", hr)?;
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
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let lib : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    /// # let scale4 = lib.functions().unwrap().find(|f| f.get_desc().unwrap().name.to_bytes() == b"scale4").unwrap();
    /// #
    /// println!("{:#?}", scale4.get_variable_by_name("scale").get_desc().unwrap());
    ///
    /// assert_eq!(
    ///     scale4.get_variable_by_name("v").get_desc().err().map(|err| err.kind()),
    ///     Some(E::FAIL) // parameter vars don't count
    /// );
    ///
    /// assert_eq!(
    ///     scale4.get_variable_by_name("\0").get_desc().err().map(|err| err.kind()),
    ///     Some(E::FAIL)
    /// );
    /// ```
    ///
    /// ### Output
    /// ```text
    /// ShaderVariableDesc {
    ///     name: "scale",
    ///     start_offset: 0,
    ///     size: 4,
    ///     flags: SVF::Used,
    ///     default_value: 0x0000000000000000,
    ///     start_texture: 4294967295,
    ///     texture_size: 0,
    ///     start_sampler: 4294967295,
    ///     sampler_size: 0,
    /// }
    /// ```
    //#allow_missing_argument_docs
    pub fn get_variable_by_name(&self, name: impl TryIntoAsCStr) -> ShaderReflectionVariable<'r> {
        let name = name.try_into().ok();
        let name = name.as_ref().map_or(cstr!("").as_cstr(), |n| n.as_cstr());
        let ptr = unsafe { self.ptr.as_ref().GetVariableByName(name) };
        unsafe { ShaderReflectionVariable::from_raw(self.phantom, ptr) }
    }
}
