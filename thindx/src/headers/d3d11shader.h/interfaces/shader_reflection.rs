use crate::*;
use crate::d3d::*;
use crate::d3d11::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflection)\]
/// ID3D11ShaderReflection
///
/// A shader-reflection interface accesses shader information.
///
/// ### See Also
/// *   [d3d::Compiler::reflect]
/// *   [d3d::Compiler::reflect11]
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
    /// // TODO
    /// ```
    pub fn get_bitwise_instruction_count(&self) -> u32 {
        unsafe { self.0.GetBitwiseInstructionCount() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getconstantbufferbyindex)\]
    /// ID3D11ShaderReflection::GetConstantBufferByIndex
    ///
    /// Get a constant buffer by index.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_constant_buffer_by_index(&self, index: u32) -> ShaderReflectionConstantBuffer {
        let ptr = unsafe { self.0.GetConstantBufferByIndex(index) };
        unsafe { ShaderReflectionConstantBuffer::from_raw(self, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getconstantbufferbyname)\]
    /// ID3D11ShaderReflection::GetConstantBufferByName
    ///
    /// Get a constant buffer by name.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_constant_buffer_by_name(&self, name: impl TryIntoAsCStr) -> ShaderReflectionConstantBuffer {
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
    /// // TODO
    /// ```
    pub fn get_conversion_instruction_count(&self) -> u32 {
        unsafe { self.0.GetConversionInstructionCount() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getdesc)\]
    /// ID3D11ShaderReflection::GetDesc
    ///
    /// Get a shader description.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_desc(&self) -> Result<ShaderDesc, Error> {
        let mut desc = ShaderDesc::default();
        let hr = unsafe { self.0.GetDesc(desc.as_mut_ptr()) };
        Error::check("ID3D11ShaderReflection::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getgsinputprimitive)\]
    /// ID3D11ShaderReflection::GetGSInputPrimitive
    ///
    /// Gets the geometry-shader input-primitive description.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_gs_input_primitive(&self) -> Primitive {
        Primitive::from_unchecked(unsafe { self.0.GetGSInputPrimitive() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getinputparameterdesc)\]
    /// ID3D11ShaderReflection::GetInputParameterDesc
    ///
    /// Get an input-parameter description for a shader.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_input_parameter_desc(&self, parameter_index: u32) -> Result<SignatureParameterDesc, Error> {
        let mut desc = SignatureParameterDesc::default();
        let hr = unsafe { self.0.GetInputParameterDesc(parameter_index, desc.as_mut_ptr()) };
        Error::check("ID3D11ShaderReflection::GetInputParameterDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getminfeaturelevel)\]
    /// ID3D11ShaderReflection::GetMinFeatureLevel
    ///
    /// Gets the minimum feature level.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_min_feature_level(&self) -> Result<FeatureLevel, Error> {
        let mut fl = 0;
        let hr = unsafe { self.0.GetMinFeatureLevel(&mut fl) };
        Error::check("ID3D11ShaderReflection::GetMinFeatureLevel", hr)?;
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
    /// // TODO
    /// ```
    pub fn get_movc_instruction_count(&self) -> u32 {
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
    /// // TODO
    /// ```
    pub fn get_mov_instruction_count(&self) -> u32 {
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
    /// // TODO
    /// ```
    pub fn get_num_interface_slots(&self) -> u32 {
        unsafe { self.0.GetNumInterfaceSlots() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getoutputparameterdesc)\]
    /// ID3D11ShaderReflection::GetOutputParameterDesc
    ///
    /// Get an output-parameter description for a shader.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_output_parameter_desc(&self, parameter_index: u32) -> Result<SignatureParameterDesc, Error> {
        let mut desc = SignatureParameterDesc::default();
        let hr = unsafe { self.0.GetOutputParameterDesc(parameter_index, desc.as_mut_ptr()) };
        Error::check("ID3D11ShaderReflection::GetOutputParameterDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getpatchconstantparameterdesc)\]
    /// ID3D11ShaderReflection::GetPatchConstantParameterDesc
    ///
    /// Get a patch-constant parameter description for a shader.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_patch_constant_parameter_desc(&self, parameter_index: u32) -> Result<SignatureParameterDesc, Error> {
        let mut desc = SignatureParameterDesc::default();
        let hr = unsafe { self.0.GetPatchConstantParameterDesc(parameter_index, desc.as_mut_ptr()) };
        Error::check("ID3D11ShaderReflection::GetPatchConstantParameterDesc", hr)?;
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
    /// // TODO
    /// ```
    pub fn get_requires_flags(&self) -> ShaderRequires {
        ShaderRequires::from_unchecked(unsafe { self.0.GetRequiresFlags() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getresourcebindingdesc)\]
    /// ID3D11ShaderReflection::GetResourceBindingDesc
    ///
    /// Get a description of how a resource is bound to a shader.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_resource_binding_desc(&self, resource_index: u32) -> Result<ShaderInputBindDesc, Error> {
        let mut desc = ShaderInputBindDesc::default();
        let hr = unsafe { self.0.GetResourceBindingDesc(resource_index, desc.as_mut_ptr()) };
        Error::check("ID3D11ShaderReflection::GetResourceBindingDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getresourcebindingdescbyname)\]
    /// ID3D11ShaderReflection::GetResourceBindingDescByName
    ///
    /// Get a description of how a resource is bound to a shader.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_resource_binding_desc_by_name(&self, name: impl TryIntoAsCStr) -> Result<ShaderInputBindDesc, Error> {
        let name = name.try_into().map_err(|e| Error::new("ID3D11ShaderReflection::GetResourceBindingDescByName", e))?;
        let mut desc = ShaderInputBindDesc::default();
        let hr = unsafe { self.0.GetResourceBindingDescByName(name.as_cstr(), desc.as_mut_ptr()) };
        Error::check("ID3D11ShaderReflection::GetResourceBindingDescByName", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getthreadgroupsize)\]
    /// ID3D11ShaderReflection::GetThreadGroupSize
    ///
    /// Retrieves the sizes, in units of threads, of the X, Y, and Z dimensions of the shader's thread-group grid.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_thread_group_size(&self) -> (u32, u32, u32) {
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
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_variable_by_name(&self, name: impl TryIntoAsCStr) -> ShaderReflectionVariable {
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
    /// // TODO
    /// ```
    pub fn is_sample_frequency_shader(&self) -> bool {
        0 != unsafe { self.0.IsSampleFrequencyShader() }
    }
}
