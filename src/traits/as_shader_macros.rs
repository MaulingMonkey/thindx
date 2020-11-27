use crate::ErrorKind;

use winapi::um::d3dcommon::D3D_SHADER_MACRO;

/// ### Safety
///
/// By implementing this trait, you promise that any returned D3D_SHADER_MACRO pointer is either null, or:
/// *   Points to an array of [D3D_SHADER_MACRO]s.
/// *   The array is terminated by a "null" (zeroed/default) [D3D_SHADER_MACRO].
/// *   All other array elements before the terminal [D3D_SHADER_MACRO] have valid, `\0` terminated `Name` and `Definition` pointers.
pub unsafe trait AsShaderMacros {
    fn as_shader_macros(&self) -> Result<*const D3D_SHADER_MACRO, ErrorKind>;
}

unsafe impl AsShaderMacros for () {
    fn as_shader_macros(&self) -> Result<*const D3D_SHADER_MACRO, ErrorKind> { Ok(std::ptr::null_mut()) }
}
