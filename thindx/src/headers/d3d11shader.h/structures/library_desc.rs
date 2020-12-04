use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_library_desc)\]
/// D3D11_LIBRARY_DESC
///
/// ### See Also
/// *   [d3d11::LibraryReflection::get_desc]
///
/// ### Example
/// ```rust
/// # use thindx::*; let d3dc = D3DCompiler::new(47).unwrap();
/// let shader = d3dc.compile_from_file(
///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
///     Compile::Debug, CompileEffect::None
/// ).unwrap().shader;
///
/// let r : d3d11::LibraryReflection = d3dc.reflect_library(shader.get_buffer()).unwrap();
/// let desc : d3d11::LibraryDesc = r.get_desc().unwrap();
/// println!("{:#?}", desc);
/// assert!(desc.function_count > 0);
/// ```
///
/// ### Output
/// ```text
/// LibraryDesc {
///     creator: Some(
///         "Microsoft (R) HLSL Shader Compiler 10.1",
///     ),
///     flags: Compile::None,
///     function_count: 1,
/// }
/// ```
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct LibraryDesc<'s> {
    pub creator:        ConstCStrPtrNullIsEmpty<'s>, // maybe never null?
    pub flags:          Compile,
    pub function_count: u32,
}

impl LibraryDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_LIBRARY_DESC {
        self as *mut Self as *mut _
    }
}

test_layout! { LibraryDesc => unsafe D3D11_LIBRARY_DESC {
    creator         => Creator,
    flags           => Flags,
    function_count  => FunctionCount,
}}
