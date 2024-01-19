#[allow(unused_imports)] use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use winapi::um::d3d11shader::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_library_desc)\]
/// D3D11_LIBRARY_DESC
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
///
/// ### See Also
/// *   [d3d11::LibraryReflection::get_desc]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct LibraryDesc<'s> {
    /// e.g. "Microsoft (R) HLSL Shader Compiler 10.1"
    pub creator:        CStrPtr<'s>, // maybe never null?
    pub flags:          Compile,
    pub function_count: u32,
}

impl LibraryDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_LIBRARY_DESC {
        self as *mut Self as *mut _
    }
}

struct_mapping! {
    #[derive(unsafe { AsRefD3D, IntoD3D })]
    // forbidden: AsRef     (could invalidate `creator`)
    // forbidden: AsMut     (could invalidate `creator`)
    // forbidden: DerefMut  (could invalidate `creator`)
    // forbidden: FromD3D   (could invalidate `creator`)
    LibraryDesc<'_> => D3D11_LIBRARY_DESC {
        creator         => Creator,
        flags           => Flags,
        function_count  => FunctionCount,
    }
}

//#cpp2rust D3D11_LIBRARY_DESC                      = d3d11::LibraryDesc
