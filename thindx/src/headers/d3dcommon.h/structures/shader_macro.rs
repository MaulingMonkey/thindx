use crate::ErrorKind;

use winapi::um::d3dcommon::D3D_SHADER_MACRO;

use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ns-d3dcommon-d3d_shader_macro)\]
/// D3D_SHADER_MACRO\[\] compatible types
///
/// ### ⚠️ Safety ⚠️
/// By implementing this trait, you promise that any returned [D3D_SHADER_MACRO] pointers are either [null], or:
/// *   Points to a valid array of [D3D_SHADER_MACRO]s.
/// *   The array is terminated by a "null" (zeroed/default) [D3D_SHADER_MACRO].
/// *   All array elements before the terminal [D3D_SHADER_MACRO] have valid `\0` terminated `Name` and `Definition` pointers.
/// *   Said array lives for at least as long as `self` remains untouched / undropped.
///
/// [D3D_SHADER_MACRO]: https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ns-d3dcommon-d3d_shader_macro
pub unsafe trait AsShaderMacros {
    /// Returns a pointer to a `std::mem::zeroed::<D3D_SHADER_MACRO>()`-terminated array of shader macros.
    fn as_shader_macros(&self) -> Result<*const D3D_SHADER_MACRO, ErrorKind>;
}

unsafe impl AsShaderMacros for () {
    fn as_shader_macros(&self) -> Result<*const D3D_SHADER_MACRO, ErrorKind> { Ok(null()) }
}

unsafe impl AsShaderMacros for Option<std::convert::Infallible> {
    fn as_shader_macros(&self) -> Result<*const D3D_SHADER_MACRO, ErrorKind> { Ok(null()) }
}

//#cpp2rust D3D_SHADER_MACRO = trait d3d::AsShaderMacros
