#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_class)\]
/// D3D_SHADER_VARIABLE_CLASS / D3D_SVC_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ShaderVariableClass(D3D_SHADER_VARIABLE_CLASS);
#[doc(hidden)] pub use ShaderVariableClass as SVC;

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { SVC => D3D_SHADER_VARIABLE_CLASS; default: Scalar == 0; Scalar, Vector, MatrixRows, MatrixColumns, Object, Struct, InterfaceClass, InterfacePointer }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl SVC { // These are enum-like
    pub const Scalar            : SVC = SVC(D3D_SVC_SCALAR); // 0
    pub const Vector            : SVC = SVC(D3D_SVC_VECTOR);
    pub const MatrixRows        : SVC = SVC(D3D_SVC_MATRIX_ROWS);
    pub const MatrixColumns     : SVC = SVC(D3D_SVC_MATRIX_COLUMNS);
    pub const Object            : SVC = SVC(D3D_SVC_OBJECT);
    pub const Struct            : SVC = SVC(D3D_SVC_STRUCT);
    pub const InterfaceClass    : SVC = SVC(D3D_SVC_INTERFACE_CLASS);
    pub const InterfacePointer  : SVC = SVC(D3D_SVC_INTERFACE_POINTER);
}

//#cpp2rust D3D_SHADER_VARIABLE_CLASS   = d3d::ShaderVariableClass

//#cpp2rust D3D_SVC_SCALAR              = d3d::SVC::Scalar
//#cpp2rust D3D_SVC_VECTOR              = d3d::SVC::Vector
//#cpp2rust D3D_SVC_MATRIX_ROWS         = d3d::SVC::MatrixRows
//#cpp2rust D3D_SVC_MATRIX_COLUMNS      = d3d::SVC::MatrixColumns
//#cpp2rust D3D_SVC_OBJECT              = d3d::SVC::Object
//#cpp2rust D3D_SVC_STRUCT              = d3d::SVC::Struct
//#cpp2rust D3D_SVC_INTERFACE_CLASS     = d3d::SVC::InterfaceClass
//#cpp2rust D3D_SVC_INTERFACE_POINTER   = d3d::SVC::InterfacePointer

//#cpp2rust D3D10_SVC_SCALAR            = d3d::SVC::Scalar
//#cpp2rust D3D10_SVC_VECTOR            = d3d::SVC::Vector
//#cpp2rust D3D10_SVC_MATRIX_ROWS       = d3d::SVC::MatrixRows
//#cpp2rust D3D10_SVC_MATRIX_COLUMNS    = d3d::SVC::MatrixColumns
//#cpp2rust D3D10_SVC_OBJECT            = d3d::SVC::Object
//#cpp2rust D3D10_SVC_STRUCT            = d3d::SVC::Struct

//#cpp2rust D3D11_SVC_INTERFACE_CLASS   = d3d::SVC::InterfaceClass
//#cpp2rust D3D11_SVC_INTERFACE_POINTER = d3d::SVC::InterfacePointer
