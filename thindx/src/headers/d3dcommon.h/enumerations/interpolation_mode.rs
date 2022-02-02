#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_interpolation_mode)\]
/// D3D_INTERPOLATION_MODE / D3D_INTERPOLATION_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct InterpolationMode(D3D_INTERPOLATION_MODE);
#[doc(hidden)] pub use InterpolationMode as Interpolation;

enumish! { Interpolation => D3D_INTERPOLATION_MODE; default: Undefined == 0; Undefined, Constant, Linear, LinearCentroid, LinearNoPerspective, LinearNoPerspectiveCentroid, LinearSample, LinearNoPerspectiveSample }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl Interpolation { // These are enum-like
    pub const Undefined                     : Interpolation = Interpolation(D3D_INTERPOLATION_UNDEFINED); // 0
    pub const Constant                      : Interpolation = Interpolation(D3D_INTERPOLATION_CONSTANT);
    pub const Linear                        : Interpolation = Interpolation(D3D_INTERPOLATION_LINEAR);
    pub const LinearCentroid                : Interpolation = Interpolation(D3D_INTERPOLATION_LINEAR_CENTROID);
    pub const LinearNoPerspective           : Interpolation = Interpolation(D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE);
    pub const LinearNoPerspectiveCentroid   : Interpolation = Interpolation(D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_CENTROID);
    pub const LinearSample                  : Interpolation = Interpolation(D3D_INTERPOLATION_LINEAR_SAMPLE);
    pub const LinearNoPerspectiveSample     : Interpolation = Interpolation(D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_SAMPLE);
}

//#cpp2rust D3D_INTERPOLATION_MODE                          = d3d::InterpolationMode

//#cpp2rust D3D_INTERPOLATION_UNDEFINED                     = d3d::Interpolation::Undefined
//#cpp2rust D3D_INTERPOLATION_CONSTANT                      = d3d::Interpolation::Constant
//#cpp2rust D3D_INTERPOLATION_LINEAR                        = d3d::Interpolation::Linear
//#cpp2rust D3D_INTERPOLATION_LINEAR_CENTROID               = d3d::Interpolation::LinearCentroid
//#cpp2rust D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE          = d3d::Interpolation::LinearNoPerspective
//#cpp2rust D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_CENTROID = d3d::Interpolation::LinearNoPerspectiveCentroid
//#cpp2rust D3D_INTERPOLATION_LINEAR_SAMPLE                 = d3d::Interpolation::LinearSample
//#cpp2rust D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_SAMPLE   = d3d::Interpolation::LinearNoPerspectiveSample
