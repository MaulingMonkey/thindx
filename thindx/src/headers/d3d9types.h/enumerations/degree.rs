#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddegreetype)\]
/// D3DDEGREETYPE
///
/// Defines the degree of the variables in the equation that describes a curve.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct DegreeType(D3DDEGREETYPE);
pub use DegreeType as Degree;

enumish! { Degree => D3DDEGREETYPE; Linear, Quadratic, Cubic, Quintic }

#[allow(non_upper_case_globals)] impl Degree { // These are enum-like
    pub const Linear        : Degree = Degree(D3DDEGREE_LINEAR); // 1
    pub const Quadratic     : Degree = Degree(D3DDEGREE_QUADRATIC);
    pub const Cubic         : Degree = Degree(D3DDEGREE_CUBIC);
    pub const Quintic       : Degree = Degree(D3DDEGREE_QUINTIC);
}

impl Degree {
    pub const fn zeroed() -> Self { Self(0) }
}

//#cpp2rust D3DDEGREETYPE       = d3d::DegreeType

//#cpp2rust D3DDEGREE_LINEAR    = d3d::Degree::Linear
//#cpp2rust D3DDEGREE_QUADRATIC = d3d::Degree::Quadratic
//#cpp2rust D3DDEGREE_CUBIC     = d3d::Degree::Cubic
//#cpp2rust D3DDEGREE_QUINTIC   = d3d::Degree::Quintic
