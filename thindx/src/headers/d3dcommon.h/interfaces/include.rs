use winapi::um::d3dcommon::ID3DInclude;
use winapi::um::d3dcompiler::D3D_COMPILE_STANDARD_FILE_INCLUDE;

use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)\]
/// ID3DInclude compatible types
///
/// ### Safety
/// By implementing this trait, you promise to return one of:
/// *   [null_mut]\(\)
/// *   [D3D_COMPILE_STANDARD_FILE_INCLUDE]
/// *   A valid, well behaved [ID3D11Include](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)
///     instance, that lives for at least as long as `self` remains untouched / undropped.
pub unsafe trait AsID3DInclude {
    /// Treat this as a raw winapi [ID3DInclude].
    fn as_id3dinclude(&self) -> *mut ID3DInclude;
}

unsafe impl AsID3DInclude for () {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { null_mut() }
}

unsafe impl AsID3DInclude for Option<void::Void> {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { null_mut() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile#parameters)\]
/// D3D_COMPILE_STANDARD_FILE_INCLUDE
///
/// This default include handler includes files that are relative to the current directory and files that are relative to
/// the directory of the initial source file. When you use [StandardFileInclude], you must specify the source file name
/// in the `source_name` parameter; the compiler will derive the initial relative directory from `source_name`.
pub struct StandardFileInclude;
unsafe impl AsID3DInclude for StandardFileInclude {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { D3D_COMPILE_STANDARD_FILE_INCLUDE }
}
