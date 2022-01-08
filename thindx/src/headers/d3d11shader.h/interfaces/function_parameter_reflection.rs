use crate::*;
use crate::d3d11::*;

use winapi::um::d3d11shader::*;

use std::marker::PhantomData;
use std::ptr::NonNull;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionparameterreflection)\]
/// ID3D11FunctionParameterReflection
///
/// A function-parameter-reflection interface accesses function-parameter info.
///
/// ### See Also
/// *   [FunctionReflection::get_function_parameter]
#[derive(Clone)] #[repr(transparent)]
pub struct FunctionParameterReflection<'r> {
    ptr:        NonNull<ID3D11FunctionParameterReflection>,
    phantom:    PhantomData<&'r ShaderReflection>,
}

impl<'r> FunctionParameterReflection<'r> {
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, ptr: *mut ID3D11FunctionParameterReflection) -> Self {
        Self {
            ptr:        NonNull::new(ptr).expect("FunctionParameterReflection should never be null"),
            phantom:    PhantomData,
        }
    }
}

impl<'r> FunctionParameterReflection<'r> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionparameterreflection-getdesc)\]
    /// ID3D11FunctionParameterReflection::GetDesc
    ///
    /// ### Errors
    /// *   [E::FAIL] if `self` is a stub object for an invalid parameter
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::new(47).unwrap();
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
    /// let v = scale4.get_function_parameter(0).get_desc().unwrap();
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
    pub fn get_desc(&self) -> Result<ParameterDesc<'r>, MethodErrorBlob> {
        let mut desc = ParameterDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetDesc(&mut desc as *mut _ as *mut _) };
        MethodErrorBlob::check("ID3D11FunctionParameterReflection::GetDesc", hr)?;
        Ok(desc)
    }
}
