use crate::*;
use crate::ctypes::*;
use crate::d3d11::*;

use winapi::um::d3d11shader::*;

use std::marker::PhantomData;
use std::ptr::NonNull;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectionconstantbuffer)\]
/// ID3D11ShaderReflectionConstantBuffer
///
/// This shader-reflection interface provides access to a constant buffer.
///
/// ### See Also
/// *   [d3d11::ShaderReflection::get_constant_buffer_by_index]
/// *   [d3d11::ShaderReflection::get_constant_buffer_by_name]
#[derive(Clone)] #[repr(transparent)]
pub struct ShaderReflectionConstantBuffer<'r> {
    ptr:        NonNull<ID3D11ShaderReflectionConstantBuffer>,
    phantom:    PhantomData<&'r LibraryReflection>,
}

impl<'r> ShaderReflectionConstantBuffer<'r> {
    /// ### ⚠️ Safety ⚠️
    /// *   `ptr` must be a valid `ID3D11ShaderReflectionConstantBuffer` for the entire lifetime of `_`
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, ptr: *mut ID3D11ShaderReflectionConstantBuffer) -> Self {
        Self {
            ptr:        NonNull::new(ptr).expect("ShaderReflectionConstantBuffer should never be null"),
            phantom:    PhantomData,
        }
    }
}

impl<'r> ShaderReflectionConstantBuffer<'r> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionconstantbuffer-getdesc)\]
    /// ID3D11ShaderReflectionConstantBuffer::GetDesc
    ///
    /// Get a constant-buffer description.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let d3dc = Compiler::load_system(47).unwrap();
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
    /// assert_eq!(E::FAIL, invalid.get_desc());
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
    pub fn get_desc(&self) -> Result<ShaderBufferDesc<'r>, Error> {
        fn_context!(d3d11::ShaderReflectionConstantBuffer::get_desc => ID3D11ShaderReflectionConstantBuffer::GetDesc);
        let mut desc = ShaderBufferDesc::default();
        fn_check_hr!(unsafe { self.ptr.as_ref().GetDesc(desc.as_mut_ptr()) })?;
        Ok(desc)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionconstantbuffer-getvariablebyindex)\]
    /// ID3D11ShaderReflectionConstantBuffer::GetVariableByIndex
    ///
    /// Get a shader-reflection variable by index.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let d3dc = Compiler::load_system(47).unwrap();
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
    /// println!("{:#?}",   valid.get_variable_by_index(0).get_desc().unwrap());
    /// assert_eq!(E::FAIL, valid.get_variable_by_index(1).get_desc());
    ///
    /// let invalid = scale4.get_constant_buffer_by_name("Nonexistant");
    /// assert_eq!(E::FAIL, invalid.get_variable_by_index(0).get_desc());
    /// assert_eq!(E::FAIL, invalid.get_variable_by_index(1).get_desc());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// ShaderVariableDesc {
    ///     name: Some(
    ///         "scale",
    ///     ),
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
    pub fn get_variable_by_index(&self, index: u32) -> ShaderReflectionVariable<'r> {
        fn_context!(d3d11::ShaderReflectionConstantBuffer::get_variable_by_index => ID3D11ShaderReflectionConstantBuffer::GetVariableByIndex);
        let ptr = unsafe { self.ptr.as_ref().GetVariableByIndex(index) };
        unsafe { ShaderReflectionVariable::from_raw(self.phantom, ptr) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionconstantbuffer-getvariablebyname)\]
    /// ID3D11ShaderReflectionConstantBuffer::GetVariableByName
    ///
    /// Get a shader-reflection variable by name.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let d3dc = Compiler::load_system(47).unwrap();
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
    /// println!("{:#?}",   valid.get_variable_by_name("scale").get_desc().unwrap());
    /// assert_eq!(E::FAIL, valid.get_variable_by_name("nope" ).get_desc());
    ///
    /// let invalid = scale4.get_constant_buffer_by_name("Nonexistant");
    /// assert_eq!(E::FAIL, invalid.get_variable_by_name("scale").get_desc());
    /// assert_eq!(E::FAIL, invalid.get_variable_by_name("nope" ).get_desc());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// ShaderVariableDesc {
    ///     name: Some(
    ///         "scale",
    ///     ),
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
        fn_context!(d3d11::ShaderReflectionConstantBuffer::get_variable_by_name => ID3D11ShaderReflectionConstantBuffer::GetVariableByName);
        let name = name.try_into().ok();
        let name = name.as_ref().map_or(cstr!("").as_cstr(), |n| n.as_cstr());
        let ptr = unsafe { self.ptr.as_ref().GetVariableByName(name) };
        unsafe { ShaderReflectionVariable::from_raw(self.phantom, ptr) }
    }
}

//#cpp2rust ID3D11ShaderReflectionConstantBuffer                            = d3d11::ShaderReflectionConstantBuffer
