#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::minwindef::UINT;
use winapi::um::d3dcommon::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_flags)\]
/// UINT / D3D_SHADER_VARIABLE_FLAGS_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ShaderVariableFlags(UINT);
#[doc(hidden)] pub use ShaderVariableFlags as SVF;

flags! { SVF => UINT; None, UserPacked, Used, InterfacePointer, InterfaceParameter }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl SVF { // These are enum-like
    pub const None                  : SVF = SVF(0);
    pub const UserPacked            : SVF = SVF(D3D_SVF_USERPACKED);
    pub const Used                  : SVF = SVF(D3D_SVF_USED);
    pub const InterfacePointer      : SVF = SVF(D3D_SVF_INTERFACE_POINTER);
    pub const InterfaceParameter    : SVF = SVF(D3D_SVF_INTERFACE_PARAMETER);
}



//#cpp2rust D3D_SHADER_VARIABLE_FLAGS       = d3d::ShaderVariableFlags

//#cpp2rust D3D_SVF_USERPACKED              = d3d::SVF::UserPacked
//#cpp2rust D3D_SVF_USED                    = d3d::SVF::Used
//#cpp2rust D3D_SVF_INTERFACE_POINTER       = d3d::SVF::InterfacePointer
//#cpp2rust D3D_SVF_INTERFACE_PARAMETER     = d3d::SVF::InterfaceParameter

//#cpp2rust D3D10_SVF_USERPACKED            = d3d::SVF::UserPacked
//#cpp2rust D3D10_SVF_USED                  = d3d::SVF::Used

//#cpp2rust D3D11_SVF_INTERFACE_POINTER     = d3d::SVF::InterfacePointer
//#cpp2rust D3D11_SVF_INTERFACE_PARAMETER   = d3d::SVF::InterfaceParameter
