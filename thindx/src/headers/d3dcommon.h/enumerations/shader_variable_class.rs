#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_class)\]
/// D3D_SHADER_VARIABLE_CLASS / D3D_SVC_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderVariableClass(D3D_SHADER_VARIABLE_CLASS);
#[doc(hidden)] pub use ShaderVariableClass as SVC;

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { SVC => D3D_SHADER_VARIABLE_CLASS; Scalar, Vector, MatrixRows, MatrixColumns, Object, Struct, InterfaceClass, InterfacePointer }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl SVC { // These are enum-like
    pub const Scalar            : SVC = SVC(D3D_SVC_SCALAR);
    pub const Vector            : SVC = SVC(D3D_SVC_VECTOR);
    pub const MatrixRows        : SVC = SVC(D3D_SVC_MATRIX_ROWS);
    pub const MatrixColumns     : SVC = SVC(D3D_SVC_MATRIX_COLUMNS);
    pub const Object            : SVC = SVC(D3D_SVC_OBJECT);
    pub const Struct            : SVC = SVC(D3D_SVC_STRUCT);
    pub const InterfaceClass    : SVC = SVC(D3D_SVC_INTERFACE_CLASS);
    pub const InterfacePointer  : SVC = SVC(D3D_SVC_INTERFACE_POINTER);
}

#[doc(hidden)] impl SVC { // Ctrl+C Ctrl+V support
    pub const SCALAR            : SVC = SVC(D3D_SVC_SCALAR);
    pub const VECTOR            : SVC = SVC(D3D_SVC_VECTOR);
    pub const MATRIX_ROWS       : SVC = SVC(D3D_SVC_MATRIX_ROWS);
    pub const MATRIX_COLUMNS    : SVC = SVC(D3D_SVC_MATRIX_COLUMNS);
    pub const OBJECT            : SVC = SVC(D3D_SVC_OBJECT);
    pub const STRUCT            : SVC = SVC(D3D_SVC_STRUCT);
    pub const INTERFACE_CLASS   : SVC = SVC(D3D_SVC_INTERFACE_CLASS);
    pub const INTERFACE_POINTER : SVC = SVC(D3D_SVC_INTERFACE_POINTER);
}

impl Default for SVC {
    fn default() -> Self { SVC(0) }
}
