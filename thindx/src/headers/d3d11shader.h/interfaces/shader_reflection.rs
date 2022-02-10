use crate::*;
use crate::ctypes::*;
use crate::d3d::*;
use crate::d3d11::*;

use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflection)\]
/// ID3D11ShaderReflection
///
/// A shader-reflection interface accesses shader information.
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// let d3dc = d3d::Compiler::load_system(47).unwrap();
/// let vs = d3dc.compile_from_file(
///     r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0",
///     d3d::Compile::Debug, d3d::CompileEffect::None
/// ).unwrap();
/// let _vs : d3d11::ShaderReflection = d3dc.reflect(&vs).unwrap();
/// let vs = d3dc.reflect11(&vs).unwrap();
/// ```
///
/// ### See Also
/// *   [d3d::Compiler::reflect]
/// *   [d3d::Compiler::reflect11]
/// *   [examples::d3dcompiler_04_reflect_shader]
#[derive(Clone)] #[repr(transparent)]
pub struct ShaderReflection(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11ShaderReflection>);

convert!(unsafe ShaderReflection => Unknown, winapi::um::d3d11shader::ID3D11ShaderReflection);

impl ShaderReflection {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getbitwiseinstructioncount)\]
    /// ID3D11ShaderReflection::GetBitwiseInstructionCount
    ///
    /// Gets the number of bitwise instructions.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let count = vs.get_bitwise_instruction_count();
    /// ```
    pub fn get_bitwise_instruction_count(&self) -> u32 {
        fn_context!(d3d11::ShaderReflection::get_bitwise_instruction_count => ID3D11ShaderReflection::GetBitwiseInstructionCount);
        unsafe { self.0.GetBitwiseInstructionCount() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getconstantbufferbyindex)\]
    /// ID3D11ShaderReflection::GetConstantBufferByIndex
    ///
    /// Get a constant buffer by index.
    ///
    /// ### Returns
    /// *   If the index is out of bounds, a stub object will still be returned.  The stub object will return [E::FAIL] from most methods.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let cb0 = vs.get_constant_buffer_by_index(0);
    /// let _ = cb0.get_desc().unwrap();
    ///
    /// let cb1 = vs.get_constant_buffer_by_index(1);
    /// assert_eq!(Some(E::FAIL), cb1.get_desc().err().map(|err| err.kind()));
    /// #
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_index(100).get_desc().err().map(|err| err.kind()));
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_index(10000).get_desc().err().map(|err| err.kind()));
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_index(1000000).get_desc().err().map(|err| err.kind()));
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_index(!0-100).get_desc().err().map(|err| err.kind()));
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_index(!0-16).get_desc().err().map(|err| err.kind()));
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_index(!0-10).get_desc().err().map(|err| err.kind()));
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_index(!0-4).get_desc().err().map(|err| err.kind()));
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_index(!0-1).get_desc().err().map(|err| err.kind()));
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_index(!0).get_desc().err().map(|err| err.kind()));
    /// ```
    //#allow_missing_argument_docs
    pub fn get_constant_buffer_by_index(&self, index: u32) -> ShaderReflectionConstantBuffer {
        fn_context!(d3d11::ShaderReflection::get_constant_buffer_by_index => ID3D11ShaderReflection::GetConstantBufferByIndex);
        let ptr = unsafe { self.0.GetConstantBufferByIndex(index) };
        unsafe { ShaderReflectionConstantBuffer::from_raw(self, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getconstantbufferbyname)\]
    /// ID3D11ShaderReflection::GetConstantBufferByName
    ///
    /// Get a constant buffer by name.
    ///
    /// ### Returns
    /// *   If the name doesn't match a valid constant buffer, a stub object will still be returned.  The stub object will return [E::FAIL] from most methods.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let cb0 = vs.get_constant_buffer_by_name("ExampleCBuffer");
    /// let _ = cb0.get_desc().unwrap();
    ///
    /// let cb1 = vs.get_constant_buffer_by_name("Invalid");
    /// assert_eq!(Some(E::FAIL), cb1.get_desc().err().map(|err| err.kind()));
    /// #
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_name("").get_desc().err().map(|err| err.kind()));
    /// # assert_eq!(Some(E::FAIL), vs.get_constant_buffer_by_name("ExampleCBuffer\0").get_desc().err().map(|err| err.kind()));
    /// ```
    //#allow_missing_argument_docs
    pub fn get_constant_buffer_by_name(&self, name: impl TryIntoAsCStr) -> ShaderReflectionConstantBuffer {
        fn_context!(d3d11::ShaderReflection::get_constant_buffer_by_name => ID3D11ShaderReflection::GetConstantBufferByName);
        let name = name.try_into().ok();
        let name = name.as_ref().map_or(cstr!("").as_cstr(), |n| n.as_cstr());
        let ptr = unsafe { self.0.GetConstantBufferByName(name) };
        unsafe { ShaderReflectionConstantBuffer::from_raw(self, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getconversioninstructioncount)\]
    /// ID3D11ShaderReflection::GetConversionInstructionCount
    ///
    /// Gets the number of conversion instructions.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let count : u32 = vs.get_conversion_instruction_count();
    /// ```
    pub fn get_conversion_instruction_count(&self) -> u32 {
        fn_context!(d3d11::ShaderReflection::get_conversion_instruction_count => ID3D11ShaderReflection::GetConversionInstructionCount);
        unsafe { self.0.GetConversionInstructionCount() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getdesc)\]
    /// ID3D11ShaderReflection::GetDesc
    ///
    /// Get a shader description.
    ///
    /// ### Errors
    /// *   ...perhaps [`E::FAIL`] if reflection was called on invalid or incompatible bytecode?
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let desc = vs.get_desc().unwrap();
    /// assert_eq!(desc.version.ty(), d3d11::ShVer::VertexShader);
    /// assert_eq!(desc.version.major(), 4);
    /// assert_eq!(desc.version.minor(), 0);
    /// ```
    pub fn get_desc(&self) -> Result<ShaderDesc, Error> {
        fn_context!(d3d11::ShaderReflection::get_desc => ID3D11ShaderReflection::GetDesc);
        let mut desc = ShaderDesc::default();
        fn_check_hr!(unsafe { self.0.GetDesc(desc.as_mut_ptr()) })?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getgsinputprimitive)\]
    /// ID3D11ShaderReflection::GetGSInputPrimitive
    ///
    /// Gets the geometry-shader input-primitive description.
    ///
    /// ### Returns
    /// *   [`d3d::Primitive::Undefined`] if `self` is not a geometry shader
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// assert_eq!(vs.get_gs_input_primitive(), d3d::Primitive::Undefined);
    /// ```
    pub fn get_gs_input_primitive(&self) -> Primitive {
        fn_context!(d3d11::ShaderReflection::get_gs_input_primitive => ID3D11ShaderReflection::GetGSInputPrimitive);
        Primitive::from_unchecked(unsafe { self.0.GetGSInputPrimitive() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getinputparameterdesc)\]
    /// ID3D11ShaderReflection::GetInputParameterDesc
    ///
    /// Get an input-parameter description for a shader.
    ///
    /// ### Errors
    /// *   [E::INVALIDARG]     - if `parameter_index` >= `self.get_desc().unwrap().input_parameters`
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// assert_eq!(0,               vs.get_input_parameter_desc(0).unwrap().semantic_index);
    /// assert_eq!(0,               vs.get_input_parameter_desc(1).unwrap().semantic_index);
    /// assert_eq!(E::INVALIDARG,   vs.get_input_parameter_desc(2).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_input_parameter_desc(100).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_input_parameter_desc(10000).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_input_parameter_desc(1000000).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_input_parameter_desc(!0-100).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_input_parameter_desc(!0-16).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_input_parameter_desc(!0-8).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_input_parameter_desc(!0-4).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_input_parameter_desc(!0-1).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_input_parameter_desc(!0).unwrap_err().kind());
    /// ```
    //#allow_missing_argument_docs
    pub fn get_input_parameter_desc(&self, parameter_index: u32) -> Result<SignatureParameterDesc, Error> {
        fn_context!(d3d11::ShaderReflection::get_input_parameter_desc => ID3D11ShaderReflection::GetInputParameterDesc);
        let mut desc = SignatureParameterDesc::default();
        fn_check_hr!(unsafe { self.0.GetInputParameterDesc(parameter_index, desc.as_mut_ptr()) })?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getminfeaturelevel)\]
    /// ID3D11ShaderReflection::GetMinFeatureLevel
    ///
    /// Gets the minimum feature level.
    ///
    /// ### Errors
    /// *   ...?
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// assert_eq!(vs.get_min_feature_level().unwrap(), d3d::FeatureLevel::_10_0);
    /// ```
    pub fn get_min_feature_level(&self) -> Result<FeatureLevel, Error> {
        fn_context!(d3d11::ShaderReflection::get_min_feature_level => ID3D11ShaderReflection::GetMinFeatureLevel);
        let mut fl = 0;
        fn_check_hr!(unsafe { self.0.GetMinFeatureLevel(&mut fl) })?;
        Ok(FeatureLevel::from_unchecked(fl))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getmovcinstructioncount)\]
    /// ID3D11ShaderReflection::GetMovcInstructionCount
    ///
    /// Gets the number of Movc instructions.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let count = vs.get_movc_instruction_count();
    /// ```
    pub fn get_movc_instruction_count(&self) -> u32 {
        fn_context!(d3d11::ShaderReflection::get_movc_instruction_count => ID3D11ShaderReflection::GetMovcInstructionCount);
        unsafe { self.0.GetMovcInstructionCount() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getmovinstructioncount)\]
    /// ID3D11ShaderReflection::GetMovInstructionCount
    ///
    /// Gets the number of Mov instructions.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let count : u32 = vs.get_mov_instruction_count();
    /// ```
    pub fn get_mov_instruction_count(&self) -> u32 {
        fn_context!(d3d11::ShaderReflection::get_mov_instruction_count => ID3D11ShaderReflection::GetMovInstructionCount);
        unsafe { self.0.GetMovInstructionCount() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getnuminterfaceslots)\]
    /// ID3D11ShaderReflection::GetNumInterfaceSlots
    ///
    /// Gets the number of interface slots in a shader.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let slots : u32 = vs.get_num_interface_slots();
    /// ```
    pub fn get_num_interface_slots(&self) -> u32 {
        fn_context!(d3d11::ShaderReflection::get_num_interface_slots => ID3D11ShaderReflection::GetNumInterfaceSlots);
        unsafe { self.0.GetNumInterfaceSlots() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getoutputparameterdesc)\]
    /// ID3D11ShaderReflection::GetOutputParameterDesc
    ///
    /// Get an output-parameter description for a shader.
    ///
    /// ### Errors
    /// *   [E::INVALIDARG]     - if `parameter_index` >= `self.get_desc().unwrap().output_parameters`
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// assert_eq!(0,               vs.get_output_parameter_desc(0).unwrap().semantic_index);
    /// assert_eq!(0,               vs.get_output_parameter_desc(1).unwrap().semantic_index);
    /// assert_eq!(E::INVALIDARG,   vs.get_output_parameter_desc(2).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_output_parameter_desc(100).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_output_parameter_desc(10000).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_output_parameter_desc(1000000).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_output_parameter_desc(!0-100).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_output_parameter_desc(!0-16).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_output_parameter_desc(!0-8).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_output_parameter_desc(!0-4).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_output_parameter_desc(!0-1).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_output_parameter_desc(!0).unwrap_err().kind());
    /// ```
    //#allow_missing_argument_docs
    pub fn get_output_parameter_desc(&self, parameter_index: u32) -> Result<SignatureParameterDesc, Error> {
        fn_context!(d3d11::ShaderReflection::get_output_parameter_desc => ID3D11ShaderReflection::GetOutputParameterDesc);
        let mut desc = SignatureParameterDesc::default();
        fn_check_hr!(unsafe { self.0.GetOutputParameterDesc(parameter_index, desc.as_mut_ptr()) })?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getpatchconstantparameterdesc)\]
    /// ID3D11ShaderReflection::GetPatchConstantParameterDesc
    ///
    /// Get a patch-constant parameter description for a shader.
    ///
    /// ### Errors
    /// *   [E::INVALIDARG]     - if `parameter_index` >= `self.get_desc().unwrap().patch_constant_parameters`
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// // TODO: proper examples
    /// assert_eq!(E::INVALIDARG,   vs.get_patch_constant_parameter_desc(2).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_patch_constant_parameter_desc(100).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_patch_constant_parameter_desc(10000).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_patch_constant_parameter_desc(1000000).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_patch_constant_parameter_desc(!0-100).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_patch_constant_parameter_desc(!0-16).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_patch_constant_parameter_desc(!0-8).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_patch_constant_parameter_desc(!0-4).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_patch_constant_parameter_desc(!0-1).unwrap_err().kind());
    /// # assert_eq!(E::INVALIDARG, vs.get_patch_constant_parameter_desc(!0).unwrap_err().kind());
    /// ```
    //#allow_missing_argument_docs
    pub fn get_patch_constant_parameter_desc(&self, parameter_index: u32) -> Result<SignatureParameterDesc, Error> {
        fn_context!(d3d11::ShaderReflection::get_patch_constant_parameter_desc => ID3D11ShaderReflection::GetPatchConstantParameterDesc);
        let mut desc = SignatureParameterDesc::default();
        fn_check_hr!(unsafe { self.0.GetPatchConstantParameterDesc(parameter_index, desc.as_mut_ptr()) })?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getrequiresflags)\]
    /// ID3D11ShaderReflection::GetRequiresFlags
    ///
    /// Gets a group of flags that indicates the requirements of a shader.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// assert_eq!(vs.get_requires_flags(), d3d::ShaderRequires::None);
    /// ```
    pub fn get_requires_flags(&self) -> ShaderRequires {
        fn_context!(d3d11::ShaderReflection::get_requires_flags => ID3D11ShaderReflection::GetRequiresFlags);
        ShaderRequires::from_unchecked(unsafe { self.0.GetRequiresFlags() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getresourcebindingdesc)\]
    /// ID3D11ShaderReflection::GetResourceBindingDesc
    ///
    /// Get a description of how a resource is bound to a shader.
    ///
    /// ### Errors
    /// *   [E::INVALIDARG]     - if `resource_index` >= `self.get_desc().unwrap().bound_resources`
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let ex = vs.get_resource_binding_desc(0).unwrap();
    /// assert_eq!(ex.ty, d3d::SIT::CBuffer);
    ///
    /// let err = vs.get_resource_binding_desc(1).err().unwrap().kind();
    /// assert_eq!(err, E::INVALIDARG);
    /// #
    /// # assert_eq!(vs.get_resource_binding_desc(100).err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc(10000).err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc(1000000).err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc(!0-100).err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc(!0-16).err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc(!0-10).err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc(!0-8).err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc(!0-4).err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc(!0-1).err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc(!0).err().unwrap().kind(), E::INVALIDARG);
    /// ```
    //#allow_missing_argument_docs
    pub fn get_resource_binding_desc(&self, resource_index: u32) -> Result<ShaderInputBindDesc, Error> {
        fn_context!(d3d11::ShaderReflection::get_resource_binding_desc => ID3D11ShaderReflection::GetResourceBindingDesc);
        let mut desc = ShaderInputBindDesc::default();
        fn_check_hr!(unsafe { self.0.GetResourceBindingDesc(resource_index, desc.as_mut_ptr()) })?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getresourcebindingdescbyname)\]
    /// ID3D11ShaderReflection::GetResourceBindingDescByName
    ///
    /// Get a description of how a resource is bound to a shader.
    ///
    /// ### Errors
    /// *   [E::INVALIDARG]         - If `name` doesn't name a valid resource binding for the shader
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let ex = vs.get_resource_binding_desc_by_name("ExampleCBuffer").unwrap();
    /// assert_eq!(ex.ty, d3d::SIT::CBuffer);
    ///
    /// let err = vs.get_resource_binding_desc_by_name("Invalid").err().unwrap().kind();
    /// assert_eq!(err, E::INVALIDARG);
    /// #
    /// # assert_eq!(vs.get_resource_binding_desc_by_name("ExampleCBuffer\0").err().unwrap().kind(), E::INVALIDARG);
    /// # assert_eq!(vs.get_resource_binding_desc_by_name("").err().unwrap().kind(), E::INVALIDARG);
    /// ```
    //#allow_missing_argument_docs
    pub fn get_resource_binding_desc_by_name(&self, name: impl TryIntoAsCStr) -> Result<ShaderInputBindDesc, Error> {
        fn_context!(d3d11::ShaderReflection::get_resource_binding_desc_by_name => ID3D11ShaderReflection::GetResourceBindingDescByName);
        let name = name.try_into();
        let name = name.as_ref().map_or(null(), |name| name.as_cstr());
        let mut desc = ShaderInputBindDesc::default();
        fn_check_hr!(unsafe { self.0.GetResourceBindingDescByName(name, desc.as_mut_ptr()) })?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getthreadgroupsize)\]
    /// ID3D11ShaderReflection::GetThreadGroupSize
    ///
    /// Retrieves the sizes, in units of threads, of the X, Y, and Z dimensions of the shader's thread-group grid.
    ///
    /// ### Returns
    /// *   `(0, 0, 0)` if not specified
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// assert_eq!(vs.get_thread_group_size(), (0, 0, 0));
    /// ```
    pub fn get_thread_group_size(&self) -> (u32, u32, u32) {
        fn_context!(d3d11::ShaderReflection::get_thread_group_size => ID3D11ShaderReflection::GetThreadGroupSize);
        let (mut x, mut y, mut z) = (0, 0, 0);
        let _total = unsafe { self.0.GetThreadGroupSize(&mut x, &mut y, &mut z) };
        debug_assert_eq!(_total, x*y*z, "ID3D11ShaderReflection::GetThreadGroupSize was supposed to return x*y*z");
        (x, y, z)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getvariablebyname)\]
    /// ID3D11ShaderReflection::GetVariableByName
    ///
    /// Gets a variable by name.
    ///
    /// ### Returns
    /// *   If `name` doesn't name a valid cbuffer variable, a stub object will still be returned.  The stub object will return [E::FAIL] from most methods.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// let tint = vs.get_variable_by_name("tint").get_desc().unwrap();
    /// assert_eq!(tint.start_offset, 0, "unexpected tint.start_offset");
    /// assert_eq!(tint.size,        16, "unexpected tint.size");
    ///
    /// let invalid = vs.get_variable_by_name("invalid").get_desc().unwrap_err();
    /// assert_eq!(invalid.kind(), E::FAIL);
    /// #
    /// # assert_eq!(vs.get_variable_by_name("tint\0").get_desc().unwrap_err().kind(), E::FAIL);
    /// # assert_eq!(vs.get_variable_by_name("").get_desc().unwrap_err().kind(), E::FAIL);
    /// ```
    //#allow_missing_argument_docs
    pub fn get_variable_by_name(&self, name: impl TryIntoAsCStr) -> ShaderReflectionVariable {
        fn_context!(d3d11::ShaderReflection::get_variable_by_name => ID3D11ShaderReflection::GetVariableByName);
        let name = name.try_into().ok();
        let name = name.as_ref().map_or(cstr!("").as_cstr(), |n| n.as_cstr());
        let ptr = unsafe { self.0.GetVariableByName(name) };
        unsafe { ShaderReflectionVariable::from_raw(self, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-issamplefrequencyshader)\]
    /// ID3D11ShaderReflection::IsSampleFrequencyShader
    ///
    /// Indicates whether a shader is a sample frequency shader.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let vs = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # let vs = d3dc.reflect11(&vs).unwrap();
    /// assert_eq!(vs.is_sample_frequency_shader(), false);
    /// ```
    pub fn is_sample_frequency_shader(&self) -> bool {
        fn_context!(d3d11::ShaderReflection::is_sample_frequency_shader => ID3D11ShaderReflection::IsSampleFrequencyShader);
        0 != unsafe { self.0.IsSampleFrequencyShader() }
    }
}

//#cpp2rust ID3D11ShaderReflection                                          = d3d11::ShaderReflection
